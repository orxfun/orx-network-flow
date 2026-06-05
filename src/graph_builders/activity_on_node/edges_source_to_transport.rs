use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};
use orx_iterable::{IntoCloningIterable, Iterable};

pub fn edges_source_to_transport<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (ori, sorted_commodities) in &prob.ori_sorted_commodities {
        if let Some(des_sorted_transports) = prob.ori_des_sorted_transports.get(ori) {
            for (_des, sorted_transports) in des_sorted_transports {
                let commodities = sorted_commodities.iter().copied();
                let transports = sorted_transports.iter().copied().into_iterable();

                connect_edges_for_od(prob, builder, indexer, commodities, transports);
            }
        }
    }
}

fn connect_edges_for_od<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    tails: impl Iterator<Item = Commodity>,
    heads: impl Iterable<Item = Transport>,
) {
    for tail in tails {
        let ready = prob.commodity_by_idx(tail).origin().time();
        let max_waiting = prob.time_bounds.max_waiting.bound(prob, tail);
        let max_departure = ready + max_waiting;
        let heads = heads.iter().filter(|&head| {
            let departure = prob.transport_by_idx(head).origin().time();
            departure >= ready && departure <= max_departure
        });
        for head in heads {
            let data = EdgeData::SourceToTransport(tail, head);
            let i = indexer.source_idx(tail).into_inner();
            let j = indexer.transport_idx(head).into_inner();
            builder.edge(data, i, j);
        }
    }
}
