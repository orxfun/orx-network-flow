use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Graph, Problem, Variant};

pub fn build_aon_graph<V: Variant>(prob: &Problem<V>) -> Graph<VertexData, EdgeData> {
    let rng = |len: usize| 0..len;

    let mut indexer = Indexer::new(prob.len_commodities(), prob.len_transports());

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
    edges_source_source_waiting(prob, &mut builder, &mut indexer);

    builder.finish()
}

fn edges_source_source_waiting<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &mut Indexer,
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
