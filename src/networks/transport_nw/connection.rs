use crate::graph::GraphBuilder;
use crate::networks::transport_nw::network_builder::TransportNwBuilder;
use crate::networks::transport_nw::{TrNwEdge, TrNwVertex};
use crate::transports::Transport;
use crate::{Problem, Variant};
use core::iter::Peekable;

pub fn add_connection_edges<V: Variant>(builder: &mut TransportNwBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();
    let p = &builder.p;

    for (x, des_sorted_transports) in &p.ori_des_sorted_transports {
        for (des, tail_sorted_transports) in des_sorted_transports {
            // tail: x => des
            if let Some(map_head_sorted_transports) = p.ori_des_sorted_transports.get(des) {
                for (y, head_sorted_transports) in map_head_sorted_transports {
                    // head: des => y

                    match x == y {
                        // no entity will take the path x->des and des->x
                        true => continue,
                        false => {
                            let tails_rev = tail_sorted_transports.iter().copied().rev();
                            let heads_rev = head_sorted_transports.iter().copied().rev().peekable();

                            connect_edges_for_od(p, builder, graph, tails_rev, heads_rev);
                        }
                    }
                }
            }
        }
    }
}

fn connect_edges_for_od<V: Variant>(
    prob: &Problem<V>,
    builder: &TransportNwBuilder<'_, V>,
    graph: &mut GraphBuilder<TrNwVertex, TrNwEdge>,
    mut tails_rev: impl Iterator<Item = Transport>,
    mut heads_rev: Peekable<impl Iterator<Item = Transport>>,
) -> Option<()> {
    // no edges once we complete traversing heads
    let mut curr_head = heads_rev.next()?;

    // connect one tail per iteration
    loop {
        // no edges once we complete traversing tails
        let tail = tails_rev.next()?;

        match find_head_for_tail(prob, &mut heads_rev, curr_head, tail) {
            Some(head) => {
                let data = TrNwEdge::Connection;
                let i = builder.transport_vidx(tail);
                let j = builder.transport_vidx(head);
                graph.edge(data, i, j);

                // same head can be assigned to prior tails
                curr_head = head;
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
    // TODO: minor speed improvement possible by caching at of tail
    let feasible = |head: Transport| prob.connectivity.can_connect(prob, tail, head);

    if !feasible(curr_head) {
        // none of the further heads can be connected to tail
        return None;
    }

    let mut curr_head = curr_head;
    loop {
        match heads_rev.peek() {
            Some(&next_head) => {
                match feasible(next_head) {
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
