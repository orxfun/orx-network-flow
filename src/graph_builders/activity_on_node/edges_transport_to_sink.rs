use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};
use orx_iterable::{IntoCloningIterable, Iterable};

pub fn edges_transport_to_sink<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (des, sorted_commodities) in &prob.des_sorted_commodities {
        if let Some(ori_sorted_transports) = prob.des_ori_sorted_transports.get(des) {
            for (_ori, sorted_transports) in ori_sorted_transports {
                let tails = sorted_transports.iter().copied().into_iterable();
                let heads = sorted_commodities.iter().copied();

                connect_edges_for_od(prob, builder, indexer, tails, heads);
            }
        }
    }
}

fn connect_edges_for_od<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    tails: impl Iterable<Item = Transport>,
    heads: impl Iterator<Item = Commodity>,
) {
    for head in heads {
        let due = prob.commodity_by_idx(head).destination().time();
        let max_earliness = prob.time_bounds.max_earliness.bound(prob, head);
        let max_lateness = prob.time_bounds.max_lateness.bound(prob, head);
        let min_due = due - max_earliness;
        let max_due = due + max_lateness;

        let feasible = |tail: &Transport| {
            let at = prob.transport_by_idx(*tail).destination().time();
            at >= min_due && at <= max_due
        };

        let tails = tails.iter().filter(feasible);

        for tail in tails {
            let data = EdgeData::TransportToSink(tail, head);
            let i = indexer.transport_idx(tail);
            let j = indexer.sink_idx(head);
            builder.edge(data, i, j);
        }
    }
}
