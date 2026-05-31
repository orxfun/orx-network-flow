use super::edges_source_to_transport::edges_source_to_transport;
use super::edges_transport_to_transport::edges_transport_to_transport;
use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::edges_transport_to_sink::edges_transport_to_sink;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
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
    edges_source_source_waiting(prob, &mut builder, &indexer);
    edges_sink_sink_waiting(prob, &mut builder, &indexer);
    edges_transport_transport_waiting(prob, &mut builder, &indexer);
    edges_source_to_transport(prob, &mut builder, &indexer);
    edges_transport_to_sink(prob, &mut builder, &indexer);
    edges_transport_to_transport(prob, &mut builder, &indexer);

    builder.finish()
}

fn edges_source_source_waiting<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (_ori, sorted_commodities) in &prob.ori_sorted_commodities {
        for pair in sorted_commodities.windows(2) {
            let s0 = indexer.source_idx(pair[0]);
            let s1 = indexer.source_idx(pair[1]);
            let data = EdgeData::SourceToSourceWait(pair[0], pair[1]);
            builder.edge(data, s0.into_inner(), s1.into_inner());
        }
    }
}

fn edges_sink_sink_waiting<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (_des, sorted_commodities) in &prob.des_sorted_commodities {
        for pair in sorted_commodities.windows(2) {
            let t0 = indexer.sink_idx(pair[0]);
            let t1 = indexer.sink_idx(pair[1]);
            let data = EdgeData::SinkToSinkWait(pair[0], pair[1]);
            builder.edge(data, t0.into_inner(), t1.into_inner());
        }
    }
}

fn edges_transport_transport_waiting<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (_ori, des_sorted_transports) in &prob.ori_des_sorted_transports {
        for (_des, sorted_transports) in des_sorted_transports {
            for pair in sorted_transports.windows(2) {
                let t0 = indexer.transport_idx(pair[0]);
                let t1 = indexer.transport_idx(pair[1]);
                let data = EdgeData::TransportToTransportWait(pair[0], pair[1]);
                builder.edge(data, t0.into_inner(), t1.into_inner());
            }
        }
    }
}
