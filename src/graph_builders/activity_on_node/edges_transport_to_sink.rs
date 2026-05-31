use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};
use core::iter::Peekable;

pub fn edges_transport_to_sink<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (des, sorted_commodities) in &prob.des_sorted_commodities {
        if let Some(ori_sorted_transports) = prob.des_ori_sorted_transports.get(des) {
            for (_ori, sorted_transports) in ori_sorted_transports {
                let mut sorted_commodities_rev = sorted_commodities.iter().rev();
                let mut sorted_transports_rev = sorted_transports.iter().rev();

                loop {
                    let more_commodities = sorted_commodities_rev.len() > 0;

                    match (more_commodities, sorted_transports_rev.next()) {
                        (false, _) => break,
                        (true, None) => break,
                        (true, Some(&t)) => {
                            let arrival = prob.transport_by_idx(t).destination().time();

                            loop {
                                match sorted_commodities_rev.next() {
                                    None => break,
                                    Some(&c) => {
                                        let due = prob.commodity_by_idx(c).destination().time();
                                        if arrival <= due {
                                            let data = EdgeData::TransportToSink(t, c);
                                            let tail = indexer.transport_idx(t).into_inner();
                                            let head = indexer.sink_idx(c).into_inner();
                                            builder.edge(data, tail, head);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// fn connect_transports_of_od<V: Variant>(
//     prob: &Problem<V>,
//     builder: &mut GraphBuilder<VertexData, EdgeData>,
//     indexer: &Indexer,
//     mut tails_rev: impl Iterator<Item = Transport>,
//     mut heads_rev: Peekable<impl Iterator<Item = Transport>>,
// ) -> Option<()> {
//     // no edges once we complete traversing transports
//     let mut curr_head = heads_rev.next()?;

//     // connect one commodity per iteration
//     loop {
//         // no edges once we complete traversing commodities
//         let tail = tails_rev.next()?;

//         match find_head_for_tail(prob, &mut heads_rev, curr_head, tail) {
//             Some(head) => {
//                 // same head can be assigned to prior tails
//                 curr_head = head;

//                 let data = EdgeData::TransportToTransport(tail, head);
//                 let tail = indexer.transport_idx(tail).into_inner();
//                 let head = indexer.transport_idx(head).into_inner();
//                 builder.edge(data, tail, head);
//             }
//             // no head for this tail, moving on to the next tail
//             None => {}
//         }
//     }
// }

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
