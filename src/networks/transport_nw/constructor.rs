use crate::graphs::VIdx;
use crate::graphs::core::{GraphCore, GraphCoreBuilder};
use crate::networks::transport_nw::nw::TrNw;
use crate::networks::transport_nw::{edge_data::TrDe, vertex_data::TrDv};
use crate::transports::Transport;
use crate::{IdxCore, Problem, Variant};
use core::iter::Peekable;

pub fn construct_tr_nw<V: Variant>(p: &Problem<V>) -> TrNw<V> {
    let mut builder = GraphCore::builder();

    for t in p.transports.indices() {
        builder.vertex(TrDv::new(t));
    }

    add_waiting_edges(p, &mut builder);
    add_connection_edges(p, &mut builder);

    builder.finish()
}

fn into_vidx(t: Transport) -> VIdx {
    VIdx::from(t.into_inner())
}

fn add_waiting_edges<V: Variant>(p: &Problem<V>, b: &mut GraphCoreBuilder<TrDv, TrDe<V>>) {
    for (_ori, des_sorted_transports) in &p.ori_des_sorted_transports {
        for (_des, sorted_transports) in des_sorted_transports {
            let tails = sorted_transports.iter().copied();
            let heads = sorted_transports.iter().copied().skip(1);
            for (tail, head) in tails.zip(heads) {
                let i = into_vidx(tail);
                let j = into_vidx(head);
                b.edge(TrDe::Waiting, i, j);
            }
        }
    }
}

fn add_connection_edges<V: Variant>(p: &Problem<V>, b: &mut GraphCoreBuilder<TrDv, TrDe<V>>) {
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

                            connect_edges_for_od(p, b, tails_rev, heads_rev);
                        }
                    }
                }
            }
        }
    }
}

fn connect_edges_for_od<V: Variant>(
    p: &Problem<V>,
    b: &mut GraphCoreBuilder<TrDv, TrDe<V>>,
    mut tails_rev: impl Iterator<Item = Transport>,
    mut heads_rev: Peekable<impl Iterator<Item = Transport>>,
) -> Option<()> {
    // no edges once we complete traversing heads
    let mut curr_head = heads_rev.next()?;

    // connect one tail per iteration
    loop {
        // no edges once we complete traversing tails
        let tail = tails_rev.next()?;

        match find_head_for_tail(p, &mut heads_rev, curr_head, tail) {
            Some(head) => {
                let capacity = p.transport_by_idx(tail).capacity();
                let data = TrDe::Transport { capacity };
                let i = into_vidx(tail);
                let j = into_vidx(head);
                b.edge(data, i, j);

                // same head can be assigned to prior tails
                curr_head = head;
            }
            // no head for this tail, moving on to the next tail
            None => {}
        }
    }
}

fn find_head_for_tail<V: Variant>(
    p: &Problem<V>,
    heads_rev: &mut Peekable<impl Iterator<Item = Transport>>,
    curr_head: Transport,
    tail: Transport,
) -> Option<Transport> {
    // TODO: minor speed improvement possible by caching at of tail
    let feasible = |head: Transport| p.connectivity.can_connect(p, tail, head);

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
