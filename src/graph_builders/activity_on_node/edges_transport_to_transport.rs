use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};
use core::iter::Peekable;

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

                    let tails_rev = tail_sorted_transports.iter().copied().rev();
                    let heads_rev = head_sorted_transports.iter().copied().rev().peekable();

                    connect_transports_of_od(prob, builder, indexer, tails_rev, heads_rev);
                }
            }
        }
    }
}

fn connect_transports_of_od<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    mut tails_rev: impl Iterator<Item = Transport>,
    mut heads_rev: Peekable<impl Iterator<Item = Transport>>,
) -> Option<()> {
    // no edges once we complete traversing transports
    let mut curr_head = heads_rev.next()?;

    // connect one commodity per iteration
    loop {
        // no edges once we complete traversing commodities
        let tail = tails_rev.next()?;

        match find_head_for_tail(prob, &mut heads_rev, curr_head, tail) {
            Some(head) => {
                // same head can be assigned to prior tails
                curr_head = head;

                let data = EdgeData::TransportToTransport(tail, head);
                let tail = indexer.transport_idx(tail).into_inner();
                let head = indexer.transport_idx(head).into_inner();
                builder.edge(data, tail, head);
            }
            // no head for this tail, moving on to the next tail
            None => {}
        }
    }
}

fn find_head_for_tail<V: Variant>(
    prob: &Problem<V>,
    heads_rev: &mut Peekable<impl Iterator<Item = Transport>>,
    curr_head: Transport,
    tail: Transport,
) -> Option<Transport> {
    // TODO: connection time must come here
    let ready = prob.transport_by_idx(tail).destination().time();
    let departure = prob.transport_by_idx(curr_head).origin().time();

    if ready > departure {
        // none of the further heads can be connected to tail
        return None;
    }

    let mut curr_head = curr_head;
    loop {
        match heads_rev.peek() {
            Some(&next_head) => {
                let departure = prob.transport_by_idx(next_head).origin().time();
                match ready <= departure {
                    // next_head can also connect to tail, so it must be preferred
                    true => curr_head = heads_rev.next().expect("is-some"),
                    // curr_head can connect to tail
                    false => return Some(curr_head),
                }
            }
            // curr_head is the earliest transport and can connect to tail
            None => return Some(curr_head),
        }
    }
}
