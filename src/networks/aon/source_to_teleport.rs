use crate::networks::aon::{AonEdge, AonVertex};
use crate::networks::aon::{sinks::Sinks, sources::Sources};
use crate::{Problem, Variant, graph::GraphBuilder};

pub fn add_source_to_teleport_edges<V: Variant>(
    builder: &mut GraphBuilder<AonVertex, AonEdge>,
    p: &Problem<V>,
    sources: &Sources,
    sinks: &Sinks,
) {
    todo!()
}
