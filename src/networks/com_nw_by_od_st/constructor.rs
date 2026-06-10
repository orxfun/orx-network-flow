use crate::graphs::VIdx;
use crate::graphs::core::{GraphCore, GraphCoreBuilder};
use crate::networks::com_nw_by_od_st::{
    edge_data::ComOdStDe, nw::ComOdStNw, vertex_data::ComOdStDv,
};
use crate::transports::Transport;
use crate::{IdxCore, Problem, Variant};
use core::iter::Peekable;

pub fn construct_tr_nw<V: Variant>(p: &Problem<V>) -> ComOdStNw {
    let mut builder = GraphCore::builder();

    // for t in p.transports.indices() {
    //     builder.vertex(TrDv::new(t));
    // }

    // add_waiting_edges(p, &mut builder);
    // add_connection_edges(p, &mut builder);

    builder.finish()
}
