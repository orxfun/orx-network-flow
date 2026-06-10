use crate::commodities::CommoditiesByOdSt;
use crate::graphs::core::{GraphCore, GraphCoreBuilder};
use crate::graphs::extended::GraphExtended;
use crate::graphs::{Edge, Graph, VIdx, Vertex};
use crate::networks::TrNw;
use crate::networks::com_by_od_st_nw::{
    edge_data::ComOdStDe, nw::ComOdStNw, vertex_data::ComOdStDv,
};
use crate::space_time::SpaceTimeOd;
use crate::transports::Transport;
use crate::{IdxCore, Problem, Variant};
use core::iter::Peekable;

pub fn construct_com_by_od_st_nw<'a, V: Variant>(
    p: &'a Problem<V>,
    tr_nw: &'a TrNw<V>,
    groups: &'a CommoditiesByOdSt<'a, V>,
    od: SpaceTimeOd,
) -> ComOdStNw<'a, V> {
    let core_vertices = tr_nw.vertices().map(|x| ComOdStDv::Transport(x.data().t));
    let core_edges = tr_nw
        .edges()
        .map(|x| ComOdStDe::TransportTransport(*x.data()));

    let mut builder = GraphExtended::builder(tr_nw, core_vertices, core_edges);

    // for t in p.transports.indices() {
    //     builder.vertex(TrDv::new(t));
    // }

    // add_waiting_edges(p, &mut builder);
    // add_connection_edges(p, &mut builder);

    builder.finish()
}

// pub enum TrDe<V: Variant> {
//     Waiting,
//     Transport { capacity: V::F },
// }
