use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::{Problem, Variant};

pub fn edges_transport_transport_waiting<V: Variant>(
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
