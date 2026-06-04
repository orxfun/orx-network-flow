use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};
use orx_iterable::{IntoCloningIterable, Iterable};

pub fn edges_transport_to_transport<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (_x, des_sorted_transports) in &prob.ori_des_sorted_transports {
        for (des, tail_sorted_transports) in des_sorted_transports {
            // tail: x => des
            if let Some(map_head_sorted_transports) = prob.ori_des_sorted_transports.get(des) {
                for (_y, head_sorted_transports) in map_head_sorted_transports {
                    // head: des => y

                    let tails = tail_sorted_transports.iter().copied();
                    let heads = head_sorted_transports.iter().copied().into_iterable();

                    connect_edges_for_od(prob, builder, indexer, tails, heads);
                }
            }
        }
    }
}

fn connect_edges_for_od<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    tails: impl Iterator<Item = Transport>,
    heads: impl Iterable<Item = Transport>,
) {
    for tail in tails {
        let at = prob.transport_by_idx(tail).destination().time();

        let feasible = |head: &Transport| {
            let min_ct = prob.time_bounds.min_conn_time.bound(prob, tail, *head);
            let max_ct = prob.time_bounds.max_conn_time.bound(prob, tail, *head);
            let dt = prob.transport_by_idx(tail).origin().time();

            dt >= at + min_ct && dt <= at + max_ct
        };

        let heads = heads.iter().filter(feasible);

        for head in heads {
            let data = EdgeData::TransportToTransport(tail, head);
            let i = indexer.transport_idx(tail).into_inner();
            let j = indexer.transport_idx(head).into_inner();
            builder.edge(data, i, j);
        }
    }
}
