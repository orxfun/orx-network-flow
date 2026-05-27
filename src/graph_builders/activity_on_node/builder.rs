use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::{Graph, Problem, Variant};

pub fn build<V: Variant>(problem: &Problem<V>) -> Graph<VertexData, EdgeData> {
    let num_sources = problem.len_commodities();
    let num_destinations = problem.len_commodities();
    let num_transports = problem.len_transports();

    let v = num_sources + num_destinations + num_transports;

    todo!()
}
