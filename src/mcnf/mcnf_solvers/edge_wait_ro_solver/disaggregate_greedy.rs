use crate::commodities::VecCommodity;
use crate::graphs::core::GraphCore;
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, VecVertex, Vertex};
use crate::mcnf::solution::{CommodityLoad, CommodityPaths, Path, PathFlow};
use crate::networks::{ConnWaitEdge, ConnWaitNw, ConnWaitVertex};
use crate::{Commodity, FlowUnit, SpaceTime, Variant, VecTransport};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

pub fn disaggregate_ro_greedy<V: Variant>(
    nw: &ConnWaitNw<'_, V>,
    ro: SpaceTime,
    edge_flow: impl Fn(EIdx) -> V::F + Copy,
    transport_loads: &mut VecTransport<Vec<CommodityLoad<V>>>,
    commodity_paths: &mut VecCommodity<CommodityPaths<V>>,
) {
    let (p, g) = (nw.p(), nw.g());

    let mut remaining_by_commodity: BTreeMap<Commodity, V::F> = BTreeMap::new();
    let mut total_remaining = <V::F as FlowUnit>::zero();

    let bypass_edge_by_commodity = nw.bypass_edge_by_commodity();
    let commodities = p.sorted_ro_commodities.value_by_key_unc(&ro);
    for &c in commodities {
        let amount = p.commodity_by_idx(c).amount();
        let bypass = bypass_edge_by_commodity[c]
            .map(edge_flow)
            .unwrap_or_default();
        let remaining = amount - bypass;

        if remaining.is_nonpos() {
            continue;
        }

        total_remaining += remaining;
        remaining_by_commodity.insert(c, remaining);
    }

    if total_remaining.is_nonpos() {
        return;
    }

    let mut dd_to_vertex: BTreeMap<SpaceTime, VIdx> = BTreeMap::new();
    let mut ro_vertex = None;
    for (v, vertex) in g.enumerated_vertices() {
        match vertex.data() {
            ConnWaitVertex::DueDes(dd) => {
                dd_to_vertex.insert(*dd, v);
            }
            ConnWaitVertex::ReadyOri(x) if *x == ro => {
                ro_vertex = Some(v);
            }
            _ => {}
        }
    }
    let Some(ro_vertex) = ro_vertex else {
        return;
    };

    let mut node_sunken_by_commodity: VecVertex<BTreeMap<Commodity, V::F>> =
        VecVertex::new_filled(g.v(), || Default::default());

    for (&commodity, &flow) in &remaining_by_commodity {
        let dd = p.commodity_by_idx(commodity).destination();
        if let Some(&v) = dd_to_vertex.get(&dd) {
            *node_sunken_by_commodity[v].entry(commodity).or_default() += flow;
        }
    }

    let mut nonzero_out_degree: VecVertex<usize> = VecVertex::new_filled(g.v(), || 0);
    for (v, vertex) in g.enumerated_vertices() {
        let count = vertex
            .out_edges()
            .filter(|&e| edge_flow(e).is_pos())
            .count();
        nonzero_out_degree[v] = count;
    }

    let mut queue = VecDeque::new();
    for v in g.vertex_indices() {
        if nonzero_out_degree[v] == 0 {
            queue.push_back(v);
        }
    }

    let mut assigned_by_edge: VecEdge<BTreeMap<Commodity, V::F>> =
        VecEdge::new_filled(g.e(), || Default::default());

    while let Some(head) = queue.pop_front() {
        let head_originating = match g.vertex(head).data() {
            ConnWaitVertex::ReadyOri(x) if *x == ro => total_remaining,
            _ => <V::F as FlowUnit>::zero(),
        };

        let in_edges: Vec<_> = g
            .vertex(head)
            .in_edges()
            .filter(|&e| edge_flow(e).is_pos())
            .filter(|&e| !matches!(g.edge(e).data(), ConnWaitEdge::Bypass(_)))
            .collect();

        for e in in_edges {
            let edge = g.edge(e);
            let tail = edge.tail();
            let edge_total = edge_flow(e);

            let assigned = assign_greedy(
                edge_total,
                &node_sunken_by_commodity[head],
                head_originating,
            );

            subtract_assignments(&mut node_sunken_by_commodity[head], &assigned);
            add_assignments(&mut node_sunken_by_commodity[tail], &assigned);
            assigned_by_edge[e] = assigned;

            nonzero_out_degree[tail] -= 1;
            if nonzero_out_degree[tail] == 0 {
                queue.push_back(tail);
            }
        }
    }

    for (t, edges) in nw.transport_edges() {
        let mut load_by_commodity: BTreeMap<Commodity, V::F> = BTreeMap::new();

        for &e in edges {
            for (&commodity, &load) in &assigned_by_edge[e] {
                if load.is_pos() {
                    *load_by_commodity.entry(commodity).or_default() += load;
                }
            }
        }

        let loads = &mut transport_loads[t];
        for (commodity, load_on_transport) in load_by_commodity {
            if load_on_transport.is_nonpos() {
                continue;
            }
            loads.push(CommodityLoad {
                commodity,
                load: load_on_transport,
            });
        }
    }

    let mut path_transports = Vec::new();
    for (&commodity, &remaining) in &remaining_by_commodity {
        if remaining.is_nonpos() {
            continue;
        }

        let dd = p.commodity_by_idx(commodity).destination();
        let Some(&dd_vertex) = dd_to_vertex.get(&dd) else {
            continue;
        };

        let mut residual = VecEdge::new_filled(g.e(), || <V::F as FlowUnit>::zero());
        for (e, assigned) in assigned_by_edge.enumerated_iter() {
            residual[e] = assigned.get(&commodity).copied().unwrap_or_default();
        }

        let mut remaining_to_extract = remaining;
        while remaining_to_extract.is_pos() {
            let Some(path_edges) = find_positive_path::<V>(g, &residual, ro_vertex, dd_vertex)
            else {
                break;
            };

            let mut path_flow = <V::F as FlowUnit>::inf();
            for &e in &path_edges {
                let x = residual[e];
                if x < path_flow {
                    path_flow = x;
                }
            }

            if path_flow.is_nonpos() {
                break;
            }

            if path_flow > remaining_to_extract {
                path_flow = remaining_to_extract;
            }

            for &e in &path_edges {
                residual[e] -= path_flow;
            }

            remaining_to_extract -= path_flow;

            debug_assert!(path_transports.is_empty());
            for &e in &path_edges {
                let head = g.edge(e).head();
                if let ConnWaitVertex::Transport(t) = g.vertex(head).data() {
                    path_transports.push(*t);
                }
            }

            if path_transports.is_empty() {
                continue;
            }

            let path = Path::drain_from(&mut path_transports);
            commodity_paths[commodity].path_flows.push(PathFlow {
                path,
                flow: path_flow,
            });
        }
    }
}

fn assign_greedy<K: Ord + Copy, F: FlowUnit>(
    total_edge_flow: F,
    head_sunken_flows: &BTreeMap<K, F>,
    originating_flow_from_head: F,
) -> BTreeMap<K, F> {
    let mut remaining_edge_flow = total_edge_flow;
    let mut remaining_head_sunken = FlowUnit::sum(head_sunken_flows.values().copied());
    let mut assigned = BTreeMap::new();

    for (&dd, &flow) in head_sunken_flows {
        if remaining_head_sunken <= originating_flow_from_head {
            break;
        }

        if remaining_edge_flow.is_nonpos() {
            break;
        }

        if flow.is_nonpos() {
            continue;
        }

        let flow_to_assign = if flow > remaining_edge_flow {
            remaining_edge_flow
        } else {
            flow
        };

        assigned.insert(dd, flow_to_assign);
        remaining_edge_flow -= flow_to_assign;
        remaining_head_sunken -= flow_to_assign;
    }

    assigned
}

fn add_assignments<K: Ord + Copy, F: FlowUnit>(dst: &mut BTreeMap<K, F>, src: &BTreeMap<K, F>) {
    for (&dd, &flow) in src {
        if flow.is_pos() {
            *dst.entry(dd).or_default() += flow;
        }
    }
}

fn subtract_assignments<K: Ord + Copy, F: FlowUnit>(
    dst: &mut BTreeMap<K, F>,
    src: &BTreeMap<K, F>,
) {
    for (&dd, &flow) in src {
        if let Some(x) = dst.get_mut(&dd) {
            *x -= flow;
        }
    }

    dst.retain(|_, flow| flow.is_pos());
}

fn find_positive_path<V: Variant>(
    g: &GraphCore<ConnWaitVertex, ConnWaitEdge>,
    residual: &VecEdge<V::F>,
    s: VIdx,
    t: VIdx,
) -> Option<Vec<EIdx>> {
    let mut visited = VecVertex::new_filled(g.v(), || false);
    let mut pred = VecVertex::new_filled(g.v(), || None);
    let mut queue = VecDeque::new();

    visited[s] = true;
    queue.push_back(s);

    while let Some(v) = queue.pop_front() {
        if v == t {
            break;
        }

        for e in g.vertex(v).out_edges() {
            if residual[e].is_nonpos() {
                continue;
            }

            if matches!(g.edge(e).data(), ConnWaitEdge::Bypass(_)) {
                continue;
            }

            let head = g.edge(e).head();
            if !visited[head] {
                visited[head] = true;
                pred[head] = Some(e);
                queue.push_back(head);
            }
        }
    }

    if !visited[t] {
        return None;
    }

    let mut edges_rev = Vec::new();
    let mut curr = t;
    while curr != s {
        let e = pred[curr]?;
        edges_rev.push(e);
        curr = g.edge(e).tail();
    }

    edges_rev.reverse();
    Some(edges_rev)
}
