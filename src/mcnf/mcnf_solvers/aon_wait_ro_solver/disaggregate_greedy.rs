use crate::commodities::VecCommodity;
use crate::graphs::core::GraphCore;
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, VecVertex, Vertex};
use crate::mcnf::solution::{CommodityLoad, CommodityPaths, Path, PathFlow};
use crate::networks::{AonWaitEdge, AonWaitNw, AonWaitVertex};
use crate::{Commodity, FlowUnit, SpaceTime, Variant, VecTransport};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

pub fn disaggregate_ro_greedy<V: Variant>(
    nw: &AonWaitNw<'_, V>,
    ro: SpaceTime,
    edge_flow: impl Fn(EIdx) -> V::F + Copy,
    transport_loads: &mut VecTransport<Vec<CommodityLoad<V>>>,
    commodity_paths: &mut VecCommodity<CommodityPaths<V>>,
) {
    let (p, g) = (nw.p(), nw.g());
    let num_commodities = p.len_commodities();

    let mut remaining_by_commodity = VecCommodity::new_filled(num_commodities, || FlowUnit::zero());
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
        remaining_by_commodity[c] = remaining;
    }

    if total_remaining.is_nonpos() {
        return;
    }

    let mut dd_to_vertex: BTreeMap<SpaceTime, VIdx> = BTreeMap::new();
    let mut ro_vertex = None;
    for (v, vertex) in g.enumerated_vertices() {
        match vertex.data() {
            AonWaitVertex::DueDes(dd) => {
                dd_to_vertex.insert(*dd, v);
            }
            AonWaitVertex::ReadyOri(x) if *x == ro => {
                ro_vertex = Some(v);
            }
            _ => {}
        }
    }
    let Some(ro_vertex) = ro_vertex else {
        return;
    };

    let mut node_sunken_by_commodity: VecVertex<VecCommodity<V::F>> =
        VecVertex::new_filled(g.v(), || {
            VecCommodity::new_filled(num_commodities, || FlowUnit::zero())
        });
    let mut node_total_sunken: VecVertex<V::F> = VecVertex::new_filled(g.v(), || FlowUnit::zero());

    for &commodity in commodities {
        let flow = remaining_by_commodity[commodity];
        if flow.is_nonpos() {
            continue;
        }

        let dd = p.commodity_by_idx(commodity).destination();
        if let Some(&v) = dd_to_vertex.get(&dd) {
            node_sunken_by_commodity[v][commodity] += flow;
            node_total_sunken[v] += flow;
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

    let mut assigned_by_edge: VecEdge<VecCommodity<V::F>> = VecEdge::new_filled(g.e(), || {
        VecCommodity::new_filled(num_commodities, || FlowUnit::zero())
    });

    while let Some(head) = queue.pop_front() {
        let head_originating = match g.vertex(head).data() {
            AonWaitVertex::ReadyOri(x) if *x == ro => total_remaining,
            _ => FlowUnit::zero(),
        };

        let in_edges: Vec<_> = g
            .vertex(head)
            .in_edges()
            .filter(|&e| edge_flow(e).is_pos())
            .filter(|&e| !matches!(g.edge(e).data(), AonWaitEdge::Bypass(_)))
            .collect();

        for e in in_edges {
            let edge = g.edge(e);
            let tail = edge.tail();
            let edge_total = edge_flow(e);

            let assigned = assign_greedy(
                edge_total,
                &node_sunken_by_commodity[head],
                node_total_sunken[head],
                commodities,
                head_originating,
            );

            let assigned_total = total_assignment(&assigned, commodities);

            subtract_assignments(&mut node_sunken_by_commodity[head], &assigned, commodities);
            add_assignments(&mut node_sunken_by_commodity[tail], &assigned, commodities);
            node_total_sunken[head] -= assigned_total;
            node_total_sunken[tail] += assigned_total;
            assigned_by_edge[e] = assigned;

            nonzero_out_degree[tail] -= 1;
            if nonzero_out_degree[tail] == 0 {
                queue.push_back(tail);
            }
        }
    }

    for (t, edges) in nw.transport_edges() {
        let mut load_by_commodity =
            VecCommodity::new_filled(num_commodities, || <V::F as FlowUnit>::zero());

        for &e in edges {
            for &commodity in commodities {
                let load = assigned_by_edge[e][commodity];
                if load.is_pos() {
                    load_by_commodity[commodity] += load;
                }
            }
        }

        let loads = &mut transport_loads[t];
        for &commodity in commodities {
            let load_on_transport = load_by_commodity[commodity];
            if load_on_transport.is_nonpos() {
                continue;
            }
            loads.push(CommodityLoad {
                commodity,
                load: load_on_transport,
            });
        }
    }

    let mut out_non_bypass_edges_by_vertex: VecVertex<Vec<EIdx>> =
        VecVertex::new_filled(g.v(), Vec::new);
    for (v, vertex) in g.enumerated_vertices() {
        let out_edges: Vec<EIdx> = vertex
            .out_edges()
            .filter(|&e| !matches!(g.edge(e).data(), AonWaitEdge::Bypass(_)))
            .collect();
        out_non_bypass_edges_by_vertex[v] = out_edges;
    }

    let mut path_transports = Vec::new();
    for &commodity in commodities {
        let remaining = remaining_by_commodity[commodity];
        if remaining.is_nonpos() {
            continue;
        }

        let dd = p.commodity_by_idx(commodity).destination();
        let Some(&dd_vertex) = dd_to_vertex.get(&dd) else {
            continue;
        };

        let mut residual = VecEdge::new_filled(g.e(), || FlowUnit::zero());
        for (e, assigned) in assigned_by_edge.enumerated_iter() {
            residual[e] = assigned[commodity];
        }

        let mut next_out_edge_pos = VecVertex::new_filled(g.v(), || 0);
        let mut blocked = VecVertex::new_filled(g.v(), || false);

        let mut remaining_to_extract = remaining;
        while remaining_to_extract.is_pos() {
            let Some(path_edges) = find_positive_path_dag::<V>(
                g,
                &out_non_bypass_edges_by_vertex,
                &residual,
                &mut next_out_edge_pos,
                &mut blocked,
                ro_vertex,
                dd_vertex,
            ) else {
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
                if let AonWaitVertex::Transport(t) = g.vertex(head).data() {
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

fn assign_greedy<F: FlowUnit>(
    total_edge_flow: F,
    head_sunken_flows: &VecCommodity<F>,
    head_total_sunken: F,
    commodities: &[Commodity],
    originating_flow_from_head: F,
) -> VecCommodity<F> {
    let mut remaining_edge_flow = total_edge_flow;
    let mut remaining_head_sunken = head_total_sunken;
    let mut assigned = VecCommodity::new_filled(head_sunken_flows.len(), || F::zero());

    for &commodity in commodities {
        if remaining_head_sunken <= originating_flow_from_head {
            break;
        }

        if remaining_edge_flow.is_nonpos() {
            break;
        }

        let flow = head_sunken_flows[commodity];
        if flow.is_nonpos() {
            continue;
        }

        let flow_to_assign = if flow > remaining_edge_flow {
            remaining_edge_flow
        } else {
            flow
        };

        assigned[commodity] = flow_to_assign;
        remaining_edge_flow -= flow_to_assign;
        remaining_head_sunken -= flow_to_assign;
    }

    assigned
}

fn total_assignment<F: FlowUnit>(assigned: &VecCommodity<F>, commodities: &[Commodity]) -> F {
    FlowUnit::sum(commodities.iter().map(|&c| assigned[c]))
}

fn add_assignments<F: FlowUnit>(
    dst: &mut VecCommodity<F>,
    src: &VecCommodity<F>,
    commodities: &[Commodity],
) {
    for &commodity in commodities {
        let flow = src[commodity];
        if flow.is_pos() {
            dst[commodity] += flow;
        }
    }
}

fn subtract_assignments<F: FlowUnit>(
    dst: &mut VecCommodity<F>,
    src: &VecCommodity<F>,
    commodities: &[Commodity],
) {
    for &commodity in commodities {
        let flow = src[commodity];
        if flow.is_pos() {
            dst[commodity] -= flow;
        }
    }
}

fn find_positive_path_dag<V: Variant>(
    g: &GraphCore<AonWaitVertex, AonWaitEdge>,
    out_non_bypass_edges_by_vertex: &VecVertex<Vec<EIdx>>,
    residual: &VecEdge<V::F>,
    next_out_edge_pos: &mut VecVertex<usize>,
    blocked: &mut VecVertex<bool>,
    s: VIdx,
    t: VIdx,
) -> Option<Vec<EIdx>> {
    if blocked[s] {
        return None;
    }

    let mut vertices = Vec::new();
    let mut path_edges = Vec::new();

    vertices.push(s);

    while let Some(&v) = vertices.last() {
        if v == t {
            return Some(path_edges);
        }

        let mut advanced = false;
        let out_edges = &out_non_bypass_edges_by_vertex[v];
        while next_out_edge_pos[v] < out_edges.len() {
            let e = out_edges[next_out_edge_pos[v]];
            if residual[e].is_nonpos() {
                next_out_edge_pos[v] += 1;
                continue;
            }

            let head = g.edge(e).head();
            if blocked[head] {
                next_out_edge_pos[v] += 1;
                continue;
            }

            path_edges.push(e);
            vertices.push(head);
            advanced = true;
            break;
        }

        if advanced {
            continue;
        }

        blocked[v] = true;
        vertices.pop();

        let Some(prev_edge) = path_edges.pop() else {
            return None;
        };
        let Some(&parent) = vertices.last() else {
            return None;
        };

        debug_assert_eq!(g.edge(prev_edge).tail(), parent);
        if next_out_edge_pos[parent] < out_non_bypass_edges_by_vertex[parent].len() {
            next_out_edge_pos[parent] += 1;
        }
    }

    None
}
