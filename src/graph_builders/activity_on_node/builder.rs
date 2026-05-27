use crate::commodities::Commodity;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::transports::Transport;
use crate::{Graph, Problem, Variant};

pub fn build<V: Variant>(prob: &Problem<V>) -> Graph<VertexData, EdgeData> {
    let rng = |len: usize| 0..len;

    let indexer = Indexer::new(prob.len_commodities(), prob.len_transports());

    let transports = rng(prob.len_transports())
        .map(Transport::from)
        .map(VertexData::Transport);
    let sources = rng(prob.len_commodities())
        .map(Commodity::from)
        .map(VertexData::Source);
    let sinks = rng(prob.len_commodities())
        .map(Commodity::from)
        .map(VertexData::Sink);
    let vertices = transports.chain(sources).chain(sinks);

    let mut builder = Graph::builder(vertices);

    builder.finish()
}
