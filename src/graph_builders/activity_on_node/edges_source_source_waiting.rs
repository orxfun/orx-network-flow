use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::{Problem, Variant};

pub fn edges_source_source_waiting<V: Variant>(
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
