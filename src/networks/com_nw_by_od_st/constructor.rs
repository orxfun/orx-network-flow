use crate::commodities::CommoditiesByOdSt;
use crate::graphs::VIdx;
use crate::graphs::core::{GraphCore, GraphCoreBuilder};
use crate::networks::TrNw;
use crate::networks::com_nw_by_od_st::{
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
) -> ComOdStNw {
    let mut builder = GraphCore::builder();

    // for t in p.transports.indices() {
    //     builder.vertex(TrDv::new(t));
    // }

    // add_waiting_edges(p, &mut builder);
    // add_connection_edges(p, &mut builder);

    builder.finish()
}
