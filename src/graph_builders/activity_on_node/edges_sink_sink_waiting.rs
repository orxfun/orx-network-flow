use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::{Problem, Variant};

pub fn edges_sink_sink_waiting<V: Variant>(
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
