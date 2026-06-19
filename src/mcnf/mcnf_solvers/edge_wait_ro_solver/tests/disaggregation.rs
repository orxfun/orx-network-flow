use super::super::disaggregate_greedy::disaggregate_ro_greedy;
use crate::graphs::{EIdx, Edge, Graph, VecEdge, Vertex};
use crate::mcnf::solution::CommodityLoad;
use crate::networks::{ConnWaitEdge, ConnWaitNwSettings, ConnWaitVertex};
use crate::{Commodity, FlowUnit, ProblemBuilder, Variant, VecTransport};
use alloc::vec::Vec;

#[derive(Clone, Copy, Default)]
struct TestVariant;

impl Variant for TestVariant {
    type S = &'static str;
    type K = usize;
    type W = &'static str;
    type V = usize;
    type T = usize;
    type F = u64;
    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}

#[test]
fn greedy_disaggregation_propagates_destinations_through_shared_upstream_transport() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "X", "B", "C"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 4);
    builder.push_commodity(1, "A", 0_i64, "C", 10_i64, 6);

    builder.push_transport(0, 0, "veh", "A", 1_i64, "X", 2_i64, 100);
    builder.push_transport(1, 1, "veh", "X", 3_i64, "B", 4_i64, 100);
    builder.push_transport(2, 2, "veh", "X", 3_i64, "C", 4_i64, 100);

    let p = builder.finish();
    let nw = p.construct_wait_nw(ConnWaitNwSettings {
        add_bypass_edges: true,
    });

    let c_b = p.commodity_ind(&0).expect("commodity 0");
    let c_c = p.commodity_ind(&1).expect("commodity 1");
    let ro = p.commodity_by_idx(c_b).origin();
    let dd_b = p.commodity_by_idx(c_b).destination();
    let dd_c = p.commodity_by_idx(c_c).destination();

    let t_ax = p.transport_ind(&0).expect("transport 0");
    let t_xb = p.transport_ind(&1).expect("transport 1");
    let t_xc = p.transport_ind(&2).expect("transport 2");

    let g = nw.g();
    let mut edge_flows = VecEdge::new_filled(g.e(), || 0_u64);

    for (e, edge) in g.enumerated_edges() {
        let tail = g.vertex(edge.tail());
        let head = g.vertex(edge.head());
        let tail_data = tail.data();
        let head_data = head.data();

        let flow = match (edge.data(), tail_data, head_data) {
            (ConnWaitEdge::Enter, ConnWaitVertex::ReadyOri(x), ConnWaitVertex::Transport(t))
                if *x == ro && *t == t_ax =>
            {
                10
            }
            (
                ConnWaitEdge::Connect,
                ConnWaitVertex::Transport(t1),
                ConnWaitVertex::Transport(t2),
            ) if *t1 == t_ax && *t2 == t_xb => 4,
            (
                ConnWaitEdge::Connect,
                ConnWaitVertex::Transport(t1),
                ConnWaitVertex::Transport(t2),
            ) if *t1 == t_ax && *t2 == t_xc => 6,
            (ConnWaitEdge::Exit, ConnWaitVertex::Transport(t), ConnWaitVertex::DueDes(dd))
                if *t == t_xb && *dd == dd_b =>
            {
                4
            }
            (ConnWaitEdge::Exit, ConnWaitVertex::Transport(t), ConnWaitVertex::DueDes(dd))
                if *t == t_xc && *dd == dd_c =>
            {
                6
            }
            _ => 0,
        };

        edge_flows[e] = flow;
    }

    let edge_flow = |e: EIdx| edge_flows[e];
    let mut transport_loads: VecTransport<Vec<CommodityLoad<TestVariant>>> =
        VecTransport::new_filled(p.len_transports(), Default::default);

    disaggregate_ro_greedy(&nw, ro, edge_flow, &mut transport_loads);

    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c_b), 4);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c_c), 6);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xb], c_b), 4);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xb], c_c), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xc], c_b), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xc], c_c), 6);
}

#[test]
fn greedy_disaggregation_splits_within_same_destination_by_remaining_amount() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "B"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 2);
    builder.push_commodity(1, "A", 0_i64, "B", 10_i64, 8);

    builder.push_transport(0, 0, "veh", "A", 1_i64, "B", 2_i64, 100);

    let p = builder.finish();
    let nw = p.construct_wait_nw(ConnWaitNwSettings {
        add_bypass_edges: true,
    });

    let c0 = p.commodity_ind(&0).expect("commodity 0");
    let c1 = p.commodity_ind(&1).expect("commodity 1");
    let ro = p.commodity_by_idx(c0).origin();
    let dd = p.commodity_by_idx(c0).destination();
    let t = p.transport_ind(&0).expect("transport 0");

    let g = nw.g();
    let mut edge_flows = VecEdge::new_filled(g.e(), || 0_u64);

    for (e, edge) in g.enumerated_edges() {
        let tail = g.vertex(edge.tail());
        let head = g.vertex(edge.head());
        let tail_data = tail.data();
        let head_data = head.data();

        let flow = match (edge.data(), tail_data, head_data) {
            (ConnWaitEdge::Enter, ConnWaitVertex::ReadyOri(x), ConnWaitVertex::Transport(tt))
                if *x == ro && *tt == t =>
            {
                10
            }
            (ConnWaitEdge::Exit, ConnWaitVertex::Transport(tt), ConnWaitVertex::DueDes(x))
                if *tt == t && *x == dd =>
            {
                10
            }
            _ => 0,
        };

        edge_flows[e] = flow;
    }

    let edge_flow = |e: EIdx| edge_flows[e];
    let mut transport_loads: VecTransport<Vec<CommodityLoad<TestVariant>>> =
        VecTransport::new_filled(p.len_transports(), Default::default);

    disaggregate_ro_greedy(&nw, ro, edge_flow, &mut transport_loads);

    assert_eq!(sum_load_for_commodity(&transport_loads[t], c0), 2);
    assert_eq!(sum_load_for_commodity(&transport_loads[t], c1), 8);
}

fn sum_load_for_commodity<V: Variant>(loads: &[CommodityLoad<V>], c: Commodity) -> V::F {
    FlowUnit::sum(loads.iter().filter(|x| x.commodity == c).map(|x| x.load))
}
