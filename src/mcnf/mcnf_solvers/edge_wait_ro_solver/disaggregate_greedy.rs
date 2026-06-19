use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, VecVertex, Vertex};
use crate::mcnf::solution::CommodityLoad;
use crate::networks::{ConnWaitNw, ConnWaitVertex};
use crate::{Commodity, FlowUnit, SpaceTime, Variant, VecTransport};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

pub fn disaggregate_ro_greedy<V: Variant>(
    nw: &ConnWaitNw<'_, V>,
    ro: SpaceTime,
    edge_flow: impl Fn(EIdx) -> V::F + Copy,
    transport_loads: &mut VecTransport<Vec<CommodityLoad<V>>>,
) {
    let (p, g) = (nw.p(), nw.g());

    let mut remaining_by_dd: BTreeMap<SpaceTime, Vec<(Commodity, V::F)>> = BTreeMap::new();
    let mut total_remaining_by_dd: BTreeMap<SpaceTime, V::F> = BTreeMap::new();
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

        let dd = p.commodity_by_idx(c).destination();
        total_remaining += remaining;
        remaining_by_dd.entry(dd).or_default().push((c, remaining));
        *total_remaining_by_dd.entry(dd).or_default() += remaining;
    }

    if total_remaining.is_nonpos() {
        return;
    }

    let mut dd_to_vertex: BTreeMap<SpaceTime, VIdx> = BTreeMap::new();
    for (v, vertex) in g.enumerated_vertices() {
        if let ConnWaitVertex::DueDes(dd) = vertex.data() {
            dd_to_vertex.insert(*dd, v);
        }
    }

    let mut node_sunken_by_dd: VecVertex<BTreeMap<SpaceTime, V::F>> =
        VecVertex::new_filled(g.v(), || Default::default());

    for (dd, flow) in total_remaining_by_dd.iter() {
        if let Some(&v) = dd_to_vertex.get(dd) {
            node_sunken_by_dd[v].insert(*dd, *flow);
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

    let mut assigned_by_edge: VecEdge<BTreeMap<SpaceTime, V::F>> =
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
            .collect();

        for e in in_edges {
            let edge = g.edge(e);
            let tail = edge.tail();
            let edge_total = edge_flow(e);

            let assigned = assign_greedy(edge_total, &node_sunken_by_dd[head], head_originating);

            subtract_assignments(&mut node_sunken_by_dd[head], &assigned);
            add_assignments(&mut node_sunken_by_dd[tail], &assigned);
            assigned_by_edge[e] = assigned;

            nonzero_out_degree[tail] -= 1;
            if nonzero_out_degree[tail] == 0 {
                queue.push_back(tail);
            }
        }
    }

    for (t, edges) in nw.transport_edges() {
        let mut load_by_dd: BTreeMap<SpaceTime, V::F> = BTreeMap::new();

        for &e in edges {
            for (&dd, &load) in &assigned_by_edge[e] {
                if load.is_pos() {
                    *load_by_dd.entry(dd).or_default() += load;
                }
            }
        }

        let loads = &mut transport_loads[t];
        for (dd, load_on_transport) in load_by_dd {
            if load_on_transport.is_nonpos() {
                continue;
            }

            let Some(remaining_of_dd) = total_remaining_by_dd.get(&dd) else {
                continue;
            };

            if remaining_of_dd.is_nonpos() {
                continue;
            }

            let Some(commodities_of_dd) = remaining_by_dd.get(&dd) else {
                continue;
            };

            for &(commodity, commodity_remaining) in commodities_of_dd {
                if commodity_remaining.is_nonpos() {
                    continue;
                }
                let load = (load_on_transport * commodity_remaining) / *remaining_of_dd;
                if load.is_pos() {
                    loads.push(CommodityLoad { commodity, load });
                }
            }
        }
    }
}

fn assign_greedy<F: FlowUnit>(
    total_edge_flow: F,
    head_sunken_flows: &BTreeMap<SpaceTime, F>,
    originating_flow_from_head: F,
) -> BTreeMap<SpaceTime, F> {
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

fn add_assignments<F: FlowUnit>(dst: &mut BTreeMap<SpaceTime, F>, src: &BTreeMap<SpaceTime, F>) {
    for (&dd, &flow) in src {
        if flow.is_pos() {
            *dst.entry(dd).or_default() += flow;
        }
    }
}

fn subtract_assignments<F: FlowUnit>(
    dst: &mut BTreeMap<SpaceTime, F>,
    src: &BTreeMap<SpaceTime, F>,
) {
    for (&dd, &flow) in src {
        if let Some(x) = dst.get_mut(&dd) {
            *x -= flow;
        }
    }

    dst.retain(|_, flow| flow.is_pos());
}
