use crate::commodities::VecCommodity;
use crate::graphs::{EIdx, EdgeRange, VIdx};
use crate::networks::space_time_nw::{
    SpaceTimeEdge, SpaceTimeGraph, SpaceTimeNwSettings, SpaceTimeVertex,
};
use crate::utils::sort::map_set_into_map_sorted_vec;
use crate::utils::std_utils::{Map, Set};
use crate::{Problem, Space, SpaceTime, Time, Variant, VecTransport};

pub struct Output {
    pub graph: SpaceTimeGraph,
    pub st_to_v: Map<SpaceTime, VIdx>,
    pub transport_arc: VecTransport<EIdx>,
    pub bypass_edges_range: EdgeRange,
    pub bypass_edge_per_commodity: VecCommodity<Option<EIdx>>,
}

pub fn construct<V: Variant>(p: &Problem<V>, settings: SpaceTimeNwSettings) -> Output {
    let mut builder = SpaceTimeGraph::builder();
    let b = &mut builder;

    // collect all relevant (space, time) pairs
    let mut space_to_times: Map<Space, Set<Time>> = Default::default();

    for t in p.transports.indices() {
        let td = p.transport_by_idx(t);
        space_to_times
            .entry(td.origin().space())
            .or_default()
            .insert(td.origin().time());
        space_to_times
            .entry(td.destination().space())
            .or_default()
            .insert(td.destination().time());
    }

    for c in p.commodities.indices() {
        let com = p.commodity_by_idx(c);
        space_to_times
            .entry(com.origin().space())
            .or_default()
            .insert(com.origin().time());
        space_to_times
            .entry(com.destination().space())
            .or_default()
            .insert(com.destination().time());
    }

    let space_to_sorted_times = map_set_into_map_sorted_vec(space_to_times);

    // create vertices for all unique space-time pairs
    let mut st_to_v: Map<SpaceTime, VIdx> = Default::default();
    for (&space, times) in &space_to_sorted_times {
        for &time in times {
            let st = SpaceTime::new(space, time);
            let v = b.vertex(SpaceTimeVertex(st));
            st_to_v.insert(st, v);
        }
    }

    // edges: wait arcs within each space (consecutive times)
    for (&space, sorted_times) in &space_to_sorted_times {
        let tails = sorted_times.iter().copied();
        let heads = sorted_times.iter().copied().skip(1);
        for (t1, t2) in tails.zip(heads) {
            let tail = *st_to_v.get(&SpaceTime::new(space, t1)).expect("exists");
            let head = *st_to_v.get(&SpaceTime::new(space, t2)).expect("exists");
            b.edge(SpaceTimeEdge::Wait, tail, head);
        }
    }

    // edges: transport arcs
    let mut transport_arc: VecTransport<EIdx> = VecTransport::new();
    for t in p.transports.indices() {
        let td = p.transport_by_idx(t);
        let tail = *st_to_v.get(&td.origin()).expect("exists");
        let head = *st_to_v.get(&td.destination()).expect("exists");
        let e = b.edge(SpaceTimeEdge::Transport(t), tail, head);
        transport_arc.push(e);
    }

    // edges: bypass arcs (one per commodity, ro → dd)
    let mut bypass_edge_per_commodity = VecCommodity::new_filled(p.len_commodities(), || None);
    let bypass_edges_range = EdgeRange::new(EIdx::from(b.e()), p.len_commodities());
    if settings.add_bypass_edges {
        for (c, com) in p.commodities.indices_values() {
            let tail = *st_to_v.get(&com.origin()).expect("exists");
            let head = *st_to_v.get(&com.destination()).expect("exists");
            let e = b.edge(SpaceTimeEdge::Bypass(c), tail, head);
            bypass_edge_per_commodity[c] = Some(e);
        }
    }

    Output {
        graph: builder.finish(),
        st_to_v,
        transport_arc,
        bypass_edges_range,
        bypass_edge_per_commodity,
    }
}
