use super::edges_source_to_sink::edges_source_to_sink;
use super::edges_source_to_transport::edges_source_to_transport;
use super::edges_transport_to_sink::edges_transport_to_sink;
use super::edges_transport_to_transport::edges_transport_to_transport;
use crate::commodities::Commodity;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::transports::Transport;
use crate::{Graph, Problem, Variant};

pub fn build_aon_graph<V: Variant>(prob: &Problem<V>) -> Graph<VertexData, EdgeData> {
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

    edges_source_to_transport(prob, &mut builder, &indexer);
    edges_transport_to_sink(prob, &mut builder, &indexer);
    edges_transport_to_transport(prob, &mut builder, &indexer);
    edges_source_to_sink(prob, &mut builder, &indexer);

    builder.finish()
}
