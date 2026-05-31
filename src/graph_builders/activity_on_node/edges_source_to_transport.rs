use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};
use core::iter::Peekable;

pub fn edges_source_to_transport<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (ori, sorted_commodities) in &prob.ori_sorted_commodities {
        if let Some(des_sorted_transports) = prob.ori_des_sorted_transports.get(ori) {
            for (_des, sorted_transports) in des_sorted_transports {
                let commodities_rev = sorted_commodities.iter().rev().copied();
                let transports_rev = sorted_transports.iter().rev().copied().peekable();

                connect_transports_of_od(prob, builder, indexer, commodities_rev, transports_rev);
            }
        }
    }
}

fn connect_transports_of_od<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    mut commodities_rev: impl Iterator<Item = Commodity>,
    mut transports_rev: Peekable<impl Iterator<Item = Transport>>,
) -> Option<()> {
    // no edges once we complete traversing transports
    let mut curr_t = transports_rev.next()?;

    // connect one commodity per iteration
    loop {
        // no edges once we complete traversing commodities
        let c = commodities_rev.next()?;

        match find_head_for_tail(prob, &mut transports_rev, curr_t, c) {
            Some(t) => {
                let data = EdgeData::SourceToTransport(c, t);
                let tail = indexer.source_idx(c).into_inner();
                let head = indexer.transport_idx(t).into_inner();
                builder.edge(data, tail, head);

                // no point in assigning same transport to prior tails, progressing
                curr_t = transports_rev.next()?;
            }
            // no transport for this commodity, moving on to the next commodity
            None => {}
        }
    }
}

fn find_head_for_tail<V: Variant>(
    prob: &Problem<V>,
    heads_rev: &mut Peekable<impl Iterator<Item = Transport>>,
    curr_head: Transport,
    tail: Commodity,
) -> Option<Transport> {
    let ready = prob.commodity_by_idx(tail).origin().time();
    let departure = prob.transport_by_idx(curr_head).origin().time();

    if ready > departure {
        // no transport can be connected to c
        return None;
    }

    let mut curr_head = curr_head;
    loop {
        match heads_rev.peek() {
            Some(&next_head) => {
                let departure = prob.transport_by_idx(next_head).origin().time();
                match ready <= departure {
                    // next_t can also connect to c, so it must be preferred
                    true => curr_head = heads_rev.next().expect("is-some"),
                    // curr_t can connect to c
                    false => return Some(curr_head),
                }
            }
            // curr_t is the earliest transport and can connect to c
            None => return Some(curr_head),
        }
    }
}
