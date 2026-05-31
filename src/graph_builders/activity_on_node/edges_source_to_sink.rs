use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::{Problem, Variant};

pub fn edges_source_to_sink<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (c, _, _data) in prob.commodities.entries() {
        let s = indexer.source_idx(c).into_inner();
        let t = indexer.sink_idx(c).into_inner();
        let data = EdgeData::SourceToSink(c);
        builder.edge(data, s, t);
    }
}
