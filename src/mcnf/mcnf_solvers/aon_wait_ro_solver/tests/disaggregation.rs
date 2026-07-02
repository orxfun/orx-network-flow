use super::super::disaggregate_greedy::disaggregate_ro_greedy;
use crate::graphs::{EIdx, Edge, Graph, VecEdge, Vertex};
use crate::mcnf::solution::{CommodityLoad, CommodityPaths, Path};
use crate::networks::{AonWaitEdge, AonWaitNwSettings, AonWaitVertex};
use crate::{
    Commodity, FlowUnit, ProblemBuilder, Variant, VecTransport, commodities::VecCommodity,
};
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
    let nw = p.construct_aon_wait_nw(AonWaitNwSettings {
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
            (AonWaitEdge::Enter, AonWaitVertex::ReadyOri(x), AonWaitVertex::Transport(t))
                if *x == ro && *t == t_ax =>
            {
                10
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_ax && *t2 == t_xb =>
            {
                4
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_ax && *t2 == t_xc =>
            {
                6
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(t), AonWaitVertex::DueDes(dd))
                if *t == t_xb && *dd == dd_b =>
            {
                4
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(t), AonWaitVertex::DueDes(dd))
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
    let mut commodity_paths: VecCommodity<CommodityPaths<TestVariant>> =
        VecCommodity::new_filled(p.len_commodities(), Default::default);

    disaggregate_ro_greedy(
        &nw,
        ro,
        edge_flow,
        &mut transport_loads,
        &mut commodity_paths,
    );

    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c_b), 4);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c_c), 6);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xb], c_b), 4);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xb], c_c), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xc], c_b), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xc], c_c), 6);

    assert_eq!(sum_path_flow(&commodity_paths[c_b]), 4);
    assert_eq!(sum_path_flow(&commodity_paths[c_c]), 6);

    assert_eq!(commodity_paths[c_b].path_flows.len(), 1);
    assert_eq!(commodity_paths[c_c].path_flows.len(), 1);

    assert_path_two_legs(&commodity_paths[c_b], t_ax, t_xb);
    assert_path_two_legs(&commodity_paths[c_c], t_ax, t_xc);
}

#[test]
fn greedy_disaggregation_splits_within_same_destination_by_remaining_amount() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "B"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 2);
    builder.push_commodity(1, "A", 0_i64, "B", 10_i64, 8);

    builder.push_transport(0, 0, "veh", "A", 1_i64, "B", 2_i64, 100);

    let p = builder.finish();
    let nw = p.construct_aon_wait_nw(AonWaitNwSettings {
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
            (AonWaitEdge::Enter, AonWaitVertex::ReadyOri(x), AonWaitVertex::Transport(tt))
                if *x == ro && *tt == t =>
            {
                10
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(tt), AonWaitVertex::DueDes(x))
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
    let mut commodity_paths: VecCommodity<CommodityPaths<TestVariant>> =
        VecCommodity::new_filled(p.len_commodities(), Default::default);

    disaggregate_ro_greedy(
        &nw,
        ro,
        edge_flow,
        &mut transport_loads,
        &mut commodity_paths,
    );

    assert_eq!(sum_load_for_commodity(&transport_loads[t], c0), 2);
    assert_eq!(sum_load_for_commodity(&transport_loads[t], c1), 8);

    assert_eq!(sum_path_flow(&commodity_paths[c0]), 2);
    assert_eq!(sum_path_flow(&commodity_paths[c1]), 8);

    assert_eq!(commodity_paths[c0].path_flows.len(), 1);
    assert_eq!(commodity_paths[c1].path_flows.len(), 1);

    assert_path_one_leg(&commodity_paths[c0], t);
    assert_path_one_leg(&commodity_paths[c1], t);
}

#[test]
fn greedy_disaggregation_handles_larger_branching_instance() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "X", "Y", "B", "C", "D"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 5);
    builder.push_commodity(1, "A", 0_i64, "C", 10_i64, 7);
    builder.push_commodity(2, "A", 0_i64, "D", 10_i64, 4);

    builder.push_transport(0, 0, "veh", "A", 1_i64, "X", 2_i64, 100);
    builder.push_transport(1, 1, "veh", "X", 3_i64, "Y", 4_i64, 100);
    builder.push_transport(2, 2, "veh", "Y", 5_i64, "B", 6_i64, 100);
    builder.push_transport(3, 3, "veh", "Y", 5_i64, "C", 6_i64, 100);
    builder.push_transport(4, 4, "veh", "X", 3_i64, "D", 4_i64, 100);

    let p = builder.finish();
    let nw = p.construct_aon_wait_nw(AonWaitNwSettings {
        add_bypass_edges: true,
    });

    let c_b = p.commodity_ind(&0).expect("commodity 0");
    let c_c = p.commodity_ind(&1).expect("commodity 1");
    let c_d = p.commodity_ind(&2).expect("commodity 2");

    let ro = p.commodity_by_idx(c_b).origin();
    let dd_b = p.commodity_by_idx(c_b).destination();
    let dd_c = p.commodity_by_idx(c_c).destination();
    let dd_d = p.commodity_by_idx(c_d).destination();

    let t_ax = p.transport_ind(&0).expect("transport 0");
    let t_xy = p.transport_ind(&1).expect("transport 1");
    let t_yb = p.transport_ind(&2).expect("transport 2");
    let t_yc = p.transport_ind(&3).expect("transport 3");
    let t_xd = p.transport_ind(&4).expect("transport 4");

    let g = nw.g();
    let mut edge_flows = VecEdge::new_filled(g.e(), || 0_u64);
    for (e, edge) in g.enumerated_edges() {
        let tail = g.vertex(edge.tail());
        let head = g.vertex(edge.head());
        let tail_data = tail.data();
        let head_data = head.data();

        let flow = match (edge.data(), tail_data, head_data) {
            (AonWaitEdge::Enter, AonWaitVertex::ReadyOri(x), AonWaitVertex::Transport(t))
                if *x == ro && *t == t_ax =>
            {
                16
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_ax && *t2 == t_xy =>
            {
                12
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_ax && *t2 == t_xd =>
            {
                4
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_xy && *t2 == t_yb =>
            {
                5
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_xy && *t2 == t_yc =>
            {
                7
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(t), AonWaitVertex::DueDes(dd))
                if *t == t_yb && *dd == dd_b =>
            {
                5
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(t), AonWaitVertex::DueDes(dd))
                if *t == t_yc && *dd == dd_c =>
            {
                7
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(t), AonWaitVertex::DueDes(dd))
                if *t == t_xd && *dd == dd_d =>
            {
                4
            }
            _ => 0,
        };
        edge_flows[e] = flow;
    }

    let edge_flow = |e: EIdx| edge_flows[e];
    let mut transport_loads: VecTransport<Vec<CommodityLoad<TestVariant>>> =
        VecTransport::new_filled(p.len_transports(), Default::default);
    let mut commodity_paths: VecCommodity<CommodityPaths<TestVariant>> =
        VecCommodity::new_filled(p.len_commodities(), Default::default);

    disaggregate_ro_greedy(
        &nw,
        ro,
        edge_flow,
        &mut transport_loads,
        &mut commodity_paths,
    );

    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c_b), 5);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c_c), 7);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c_d), 4);

    assert_eq!(sum_load_for_commodity(&transport_loads[t_xy], c_b), 5);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xy], c_c), 7);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xy], c_d), 0);

    assert_eq!(sum_load_for_commodity(&transport_loads[t_yb], c_b), 5);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_yb], c_c), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_yb], c_d), 0);

    assert_eq!(sum_load_for_commodity(&transport_loads[t_yc], c_b), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_yc], c_c), 7);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_yc], c_d), 0);

    assert_eq!(sum_load_for_commodity(&transport_loads[t_xd], c_b), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xd], c_c), 0);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xd], c_d), 4);

    assert_eq!(sum_path_flow(&commodity_paths[c_b]), 5);
    assert_eq!(sum_path_flow(&commodity_paths[c_c]), 7);
    assert_eq!(sum_path_flow(&commodity_paths[c_d]), 4);

    assert_eq!(commodity_paths[c_b].path_flows.len(), 1);
    assert_eq!(commodity_paths[c_c].path_flows.len(), 1);
    assert_eq!(commodity_paths[c_d].path_flows.len(), 1);

    assert_path_three_legs(&commodity_paths[c_b], t_ax, t_xy, t_yb);
    assert_path_three_legs(&commodity_paths[c_c], t_ax, t_xy, t_yc);
    assert_path_two_legs(&commodity_paths[c_d], t_ax, t_xd);
}

#[test]
fn greedy_disaggregation_extracts_multiple_paths_for_single_commodity() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "X", "Y", "B"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 10);

    builder.push_transport(0, 0, "veh", "A", 1_i64, "X", 2_i64, 100);
    builder.push_transport(1, 1, "veh", "A", 1_i64, "Y", 2_i64, 100);
    builder.push_transport(2, 2, "veh", "X", 3_i64, "B", 4_i64, 100);
    builder.push_transport(3, 3, "veh", "Y", 3_i64, "B", 4_i64, 100);

    let p = builder.finish();
    let nw = p.construct_aon_wait_nw(AonWaitNwSettings {
        add_bypass_edges: true,
    });

    let c = p.commodity_ind(&0).expect("commodity 0");
    let ro = p.commodity_by_idx(c).origin();
    let dd = p.commodity_by_idx(c).destination();

    let t_ax = p.transport_ind(&0).expect("transport 0");
    let t_ay = p.transport_ind(&1).expect("transport 1");
    let t_xb = p.transport_ind(&2).expect("transport 2");
    let t_yb = p.transport_ind(&3).expect("transport 3");

    let g = nw.g();
    let mut edge_flows = VecEdge::new_filled(g.e(), || 0_u64);
    for (e, edge) in g.enumerated_edges() {
        let tail = g.vertex(edge.tail());
        let head = g.vertex(edge.head());
        let tail_data = tail.data();
        let head_data = head.data();

        let flow = match (edge.data(), tail_data, head_data) {
            (AonWaitEdge::Enter, AonWaitVertex::ReadyOri(x), AonWaitVertex::Transport(t))
                if *x == ro && *t == t_ax =>
            {
                4
            }
            (AonWaitEdge::Enter, AonWaitVertex::ReadyOri(x), AonWaitVertex::Transport(t))
                if *x == ro && *t == t_ay =>
            {
                6
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_ax && *t2 == t_xb =>
            {
                4
            }
            (AonWaitEdge::Connect, AonWaitVertex::Transport(t1), AonWaitVertex::Transport(t2))
                if *t1 == t_ay && *t2 == t_yb =>
            {
                6
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(t), AonWaitVertex::DueDes(x))
                if *t == t_xb && *x == dd =>
            {
                4
            }
            (AonWaitEdge::Exit, AonWaitVertex::Transport(t), AonWaitVertex::DueDes(x))
                if *t == t_yb && *x == dd =>
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
    let mut commodity_paths: VecCommodity<CommodityPaths<TestVariant>> =
        VecCommodity::new_filled(p.len_commodities(), Default::default);

    disaggregate_ro_greedy(
        &nw,
        ro,
        edge_flow,
        &mut transport_loads,
        &mut commodity_paths,
    );

    assert_eq!(sum_load_for_commodity(&transport_loads[t_ax], c), 4);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_ay], c), 6);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_xb], c), 4);
    assert_eq!(sum_load_for_commodity(&transport_loads[t_yb], c), 6);

    assert_eq!(sum_path_flow(&commodity_paths[c]), 10);
    assert_eq!(commodity_paths[c].path_flows.len(), 2);

    let mut found_ax_xb = false;
    let mut found_ay_yb = false;

    for pf in &commodity_paths[c].path_flows {
        match pf.path {
            Path::TwoLegs([a, b]) if a == t_ax && b == t_xb => {
                assert_eq!(pf.flow, 4);
                found_ax_xb = true;
            }
            Path::TwoLegs([a, b]) if a == t_ay && b == t_yb => {
                assert_eq!(pf.flow, 6);
                found_ay_yb = true;
            }
            _ => panic!("unexpected path extracted"),
        }
    }

    assert!(found_ax_xb);
    assert!(found_ay_yb);
}

fn sum_load_for_commodity<V: Variant>(loads: &[CommodityLoad<V>], c: Commodity) -> V::F {
    FlowUnit::sum(loads.iter().filter(|x| x.commodity == c).map(|x| x.load))
}

fn sum_path_flow<V: Variant>(paths: &CommodityPaths<V>) -> V::F {
    FlowUnit::sum(paths.path_flows.iter().map(|x| x.flow))
}

fn assert_path_one_leg<V: Variant>(paths: &CommodityPaths<V>, t: crate::Transport) {
    let p = &paths.path_flows[0].path;
    match p {
        Path::OneLeg([x]) => assert_eq!(*x, t),
        _ => panic!("expected one-leg path"),
    }
}

fn assert_path_two_legs<V: Variant>(
    paths: &CommodityPaths<V>,
    t1: crate::Transport,
    t2: crate::Transport,
) {
    let p = &paths.path_flows[0].path;
    match p {
        Path::TwoLegs([a, b]) => {
            assert_eq!(*a, t1);
            assert_eq!(*b, t2);
        }
        _ => panic!("expected two-leg path"),
    }
}

fn assert_path_three_legs<V: Variant>(
    paths: &CommodityPaths<V>,
    t1: crate::Transport,
    t2: crate::Transport,
    t3: crate::Transport,
) {
    let p = &paths.path_flows[0].path;
    match p {
        Path::ThreeLegs([a, b, c]) => {
            assert_eq!(*a, t1);
            assert_eq!(*b, t2);
            assert_eq!(*c, t3);
        }
        _ => panic!("expected three-leg path"),
    }
}
