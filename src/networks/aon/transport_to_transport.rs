use crate::graph::GraphBuilder;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use crate::networks::aon::{AonEdge, AonVertex};
use crate::transports::Transport;
use crate::{Problem, Variant};
use orx_iterable::{IntoCloningIterable, Iterable};

pub fn add_transport_to_transport_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();
    let p = &builder.p;

    for (_x, des_sorted_transports) in &p.ori_des_sorted_transports {
        for (des, tail_sorted_transports) in des_sorted_transports {
            // tail: x => des
            if let Some(map_head_sorted_transports) = p.ori_des_sorted_transports.get(des) {
                for (_y, head_sorted_transports) in map_head_sorted_transports {
                    // head: des => y

                    let tails = tail_sorted_transports.iter().copied();
                    let heads = head_sorted_transports.iter().copied().into_iterable();

                    connect_edges_for_od(p, builder, graph, tails, heads);
                }
            }
        }
    }
}

fn connect_edges_for_od<V: Variant>(
    prob: &Problem<V>,
    builder: &AonNetworkBuilder<'_, V>,
    graph: &mut GraphBuilder<AonVertex, AonEdge>,
    tails: impl Iterator<Item = Transport>,
    heads: impl Iterable<Item = Transport>,
) {
    for tail in tails {
        let at = prob.transport_by_idx(tail).destination().time();

        let feasible = |head: &Transport| {
            let min_ct = prob.time_bounds.min_conn_time.bound(prob, tail, *head);
            let max_ct = prob.time_bounds.max_conn_time.bound(prob, tail, *head);
            let dt = prob.transport_by_idx(*head).origin().time();

            dt >= at + min_ct && dt <= at + max_ct
        };

        let heads = heads.iter().filter(feasible);

        for head in heads {
            let data = AonEdge::TransportTransport;
            let i = builder.transport_vidx(tail);
            let j = builder.transport_vidx(head);
            graph.edge(data, i, j);
        }
    }
}
