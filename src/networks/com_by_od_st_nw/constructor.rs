use crate::commodities::{CommoditiesByOdSt, Commodity, CommodityData};
use crate::flow_units::FlowUnit;
use crate::graphs::core::{GraphCore, GraphCoreBuilder};
use crate::graphs::extended::{GraphExtended, GraphExtendedBuilder};
use crate::graphs::{Edge, Graph, VIdx, Vertex};
use crate::indices::IdxMapSubset;
use crate::networks::TrNw;
use crate::networks::com_by_od_st_nw::{
    edge_data::ComOdStDe, nw::ComOdStNw, vertex_data::ComOdStDv,
};
use crate::networks::transport_nw::{TrDe, TrDv};
use crate::space_time::{SpaceTime, SpaceTimeOd};
use crate::transports::Transport;
use crate::{IdxCore, Problem, Variant};
use alloc::vec::Vec;
use core::iter::Peekable;

pub fn construct_com_by_od_st_nw<'a, V: Variant>(
    p: &'a Problem<V>,
    tr_nw: &'a TrNw<V>,
    groups: &'a CommoditiesByOdSt<'a, V>,
) -> ComOdStNw<'a, V> {
    let core_vertices = tr_nw.vertices().map(|x| ComOdStDv::Transport(x.data().t));
    let core_edges = tr_nw
        .edges()
        .map(|x| ComOdStDe::TransportTransport(*x.data()));

    let mut builder = GraphExtended::builder(tr_nw, core_vertices, core_edges);
    let b = &mut builder;

    for (od_st, group) in groups.iter() {
        add_source_vertices(b, od_st, group);
    }

    // for t in p.transports.indices() {
    //     builder.vertex(TrDv::new(t));
    // }

    // add_waiting_edges(p, &mut builder);
    // add_connection_edges(p, &mut builder);

    builder.finish()
}

fn add_source_vertices<V: Variant>(
    b: &mut GraphExtendedBuilder<'_, TrNw<V>, ComOdStDv<V>, ComOdStDe<V>>,
    od_st: &SpaceTimeOd,
    group: &IdxMapSubset<'_, V::K, CommodityData<V>, Commodity>,
) {
    let total_amount = FlowUnit::sum(group.values().map(|x| x.amount()));
    let data = ComOdStDv::OriSt(*od_st, total_amount);
    b.vertex(data);
}
