use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::{Graph, Problem, Variant};

pub fn build<V: Variant>(prob: &Problem<V>) -> Graph<VertexData, EdgeData> {
    let indexer = Indexer::new(prob.len_commodities(), prob.len_transports());

    let v = indexer.num_vertices();

    todo!()
}
