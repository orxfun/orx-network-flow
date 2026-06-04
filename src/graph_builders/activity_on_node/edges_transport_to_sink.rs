use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};
use core::iter::Peekable;
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
        //
    }
}

fn connect_edges_for_od_deprecated<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    mut tails_rev: impl Iterator<Item = Transport>,
    mut heads_rev: Peekable<impl Iterator<Item = Commodity>>,
) -> Option<()> {
    // no edges once we complete traversing heads
    let mut curr_head = heads_rev.next()?;

    // connect one tail per iteration
    loop {
        // no edges once we complete traversing tails
        let tail = tails_rev.next()?;

        match find_head_for_tail_deprecated(prob, &mut heads_rev, curr_head, tail) {
            Some(head) => {
                let data = EdgeData::TransportToSink(tail, head);
                let i = indexer.transport_idx(tail).into_inner();
                let j = indexer.sink_idx(head).into_inner();
                builder.edge(data, i, j);

                // same head can be assigned to prior tails
                curr_head = head;
            }
            // no head for this tail, moving on to the next tail
            None => {}
        }
    }
}

fn find_head_for_tail_deprecated<V: Variant>(
    prob: &Problem<V>,
    heads_rev: &mut Peekable<impl Iterator<Item = Commodity>>,
    curr_head: Commodity,
    tail: Transport,
) -> Option<Commodity> {
    // TODO: lateness handling must come here
    let ready = prob.transport_by_idx(tail).destination().time();
    let departure = prob.commodity_by_idx(curr_head).destination().time();

    if ready > departure {
        // none of the further heads can be connected to tail
        return None;
    }

    let mut curr_head = curr_head;
    loop {
        match heads_rev.peek() {
            Some(&next_head) => {
                let departure = prob.commodity_by_idx(next_head).destination().time();
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
