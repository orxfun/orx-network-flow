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
                let commodities = sorted_commodities.iter().copied();
                let transports = sorted_transports.iter().copied();

                connect_edges_for_od(prob, builder, indexer, commodities, transports);
            }
        }
    }
}

fn connect_edges_for_od<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    mut tails: impl Iterator<Item = Commodity>,
    mut heads: impl Iterator<Item = Transport>,
) -> Option<()> {
    // no edges once we complete traversing heads
    let mut curr_head = heads.next()?;

    // connect one tail per iteration
    loop {
        // no edges once we complete traversing tails
        let tail = tails.next()?;

        // match find_head_for_tail_deprecated(prob, &mut heads, curr_head, tail) {
        //     Some(head) => {
        //         let data = EdgeData::SourceToTransport(tail, head);
        //         let i = indexer.source_idx(tail).into_inner();
        //         let j = indexer.transport_idx(head).into_inner();
        //         builder.edge(data, i, j);

        //         // no point in assigning same transport to prior tails, progressing
        //         curr_head = heads.next()?;
        //     }
        //     // no transport for this commodity, moving on to the next commodity
        //     None => {}
        // }
        todo!()
    }
}

fn connect_edges_for_od_deprecated<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    mut tails_rev: impl Iterator<Item = Commodity>,
    mut heads_rev: Peekable<impl Iterator<Item = Transport>>,
) -> Option<()> {
    // no edges once we complete traversing heads
    let mut curr_head = heads_rev.next()?;

    // connect one tail per iteration
    loop {
        // no edges once we complete traversing tails
        let tail = tails_rev.next()?;

        match find_head_for_tail_deprecated(prob, &mut heads_rev, curr_head, tail) {
            Some(head) => {
                let data = EdgeData::SourceToTransport(tail, head);
                let i = indexer.source_idx(tail).into_inner();
                let j = indexer.transport_idx(head).into_inner();
                builder.edge(data, i, j);

                // no point in assigning same transport to prior tails, progressing
                curr_head = heads_rev.next()?;
            }
            // no transport for this commodity, moving on to the next commodity
            None => {}
        }
    }
}

fn find_head_for_tail_deprecated<V: Variant>(
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
