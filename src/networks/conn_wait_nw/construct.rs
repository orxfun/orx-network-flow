use crate::graphs::{VIdx, core::GraphCoreBuilder};
use crate::networks::ConnWaitNwSettings;
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitGraph, ConnWaitVertex};
use crate::utils::sort::map_set_into_map_sorted_vec;
use crate::utils::std_utils::{Map, Set};
use crate::{IdxCore, Problem, Space, SpaceTime, Time, Transport, Variant};
use core::iter::Peekable;

pub struct Output {
    pub graph: ConnWaitGraph,
    pub ro_to_v: Map<SpaceTime, VIdx>,
    pub dd_to_v: Map<SpaceTime, VIdx>,
}

pub fn construct_graph<V: Variant>(p: &Problem<V>, settings: ConnWaitNwSettings) -> Output {
    let mut builder = ConnWaitGraph::builder();
    let b = &mut builder;

    // vertices: transport
    for t in p.transports.indices() {
        b.vertex(ConnWaitVertex::Transport(t));
    }

    // vertices: ready-ori
    let mut ro_to_v: Map<SpaceTime, VIdx> = Default::default();
    let mut ori_to_sorted_ready: Map<Space, Set<Time>> = Default::default();
    for ori in &p.sorted_commodity_origins {
        if let Some(sorted_commodities) = p.ori_sorted_commodities.get(ori) {
            for &c in sorted_commodities {
                let com = p.commodity_by_idx(c);
                debug_assert_eq!(*ori, com.origin().space());

                let sorted_ready_set = ori_to_sorted_ready.entry(*ori).or_default();
                sorted_ready_set.insert(com.origin().time());

                let ro = com.origin();
                if !ro_to_v.contains_key(&ro) {
                    let v = b.vertex(ConnWaitVertex::ReadyOri(ro));
                    ro_to_v.insert(ro, v);
                }
            }
        }
    }
    let ori_to_sorted_ready = map_set_into_map_sorted_vec(ori_to_sorted_ready);

    // vertices: due-des
    let mut dd_to_v: Map<SpaceTime, VIdx> = Default::default();
    let mut des_to_sorted_due: Map<Space, Set<Time>> = Default::default();
    for des in &p.sorted_commodity_destinations {
        if let Some(sorted_commodities) = p.des_sorted_commodities.get(des) {
            for &c in sorted_commodities {
                let com = p.commodity_by_idx(c);
                debug_assert_eq!(*des, com.destination().space());

                let sorted_due_set = des_to_sorted_due.entry(*des).or_default();
                sorted_due_set.insert(com.destination().time());

                let ro = com.destination();
                if !dd_to_v.contains_key(&ro) {
                    let v = b.vertex(ConnWaitVertex::DueDes(ro));
                    dd_to_v.insert(ro, v);
                }
            }
        }
    }
    let des_to_sorted_due = map_set_into_map_sorted_vec(des_to_sorted_due);

    // edges: t-t wait
    for (_, des_transports) in &p.ori_des_sorted_transports {
        for (_, transports) in des_transports {
            let tails = transports.iter().copied();
            let heads = transports.iter().copied().skip(1);
            for (tail, head) in tails.zip(heads) {
                b.edge(ConnWaitEdge::Wait, t_into_v(tail), t_into_v(head));
            }
        }
    }

    // edges: t-t connect
    for (x, des_sorted_transports) in &p.ori_des_sorted_transports {
        for (des, tail_sorted_transports) in des_sorted_transports {
            // tail: x => des
            if let Some(map_head_sorted_transports) = p.ori_des_sorted_transports.get(des) {
                for (y, head_sorted_transports) in map_head_sorted_transports {
                    // head: des => y

                    match p.connectivity.can_connect_spatially(p, [*x, *des, *y]) {
                        false => continue,
                        true => {
                            let tails_rev = tail_sorted_transports.iter().copied().rev();
                            let heads_rev = head_sorted_transports.iter().copied().rev().peekable();
                            conn_t_t(p, b, tails_rev, heads_rev);
                        }
                    }
                }
            }
        }
    }

    // edges: ro-t connect

    for (&ori, sorted_ready) in &ori_to_sorted_ready {
        if let Some(des_sorted_transports) = p.ori_des_sorted_transports.get(&ori) {
            for (_, sorted_transports) in des_sorted_transports {
                let map_tail = |r: &Time| {
                    let ro = SpaceTime::new(ori, *r);
                    let v = *ro_to_v.get(&ro).expect("exists");
                    (*r, v)
                };
                let tails_rev = sorted_ready.iter().rev().map(map_tail);
                let heads_rev = sorted_transports.iter().copied().rev().peekable();
                conn_ro_t(p, b, tails_rev, heads_rev);
            }
        }
    }

    // edges: t-dd connect
    for (&des, sorted_due) in &des_to_sorted_due {
        if let Some(ori_sorted_transports) = p.des_ori_sorted_transports.get(&des) {
            for (_, sorted_transports) in ori_sorted_transports {
                let map_head = |d: &Time| {
                    let dd = SpaceTime::new(des, *d);
                    let v = *dd_to_v.get(&dd).expect("exists");
                    (*d, v)
                };
                let tails = sorted_transports.iter().copied();
                let heads = sorted_due.iter().map(map_head);
                conn_t_dd(p, b, tails, heads);
            }
        }
    }

    // edges: ro-dd bypass
    if settings.add_bypass_edges {
        for (c, com) in p.commodities.indices_values() {
            let ro = *ro_to_v.get(&com.origin()).expect("exists");
            let dd = *dd_to_v.get(&com.destination()).expect("exists");
            b.edge(ConnWaitEdge::Bypass(c), ro, dd);
        }
    }

    let graph = builder.finish();

    Output {
        graph,
        ro_to_v,
        dd_to_v,
    }
}

fn conn_t_dd<V: Variant>(
    p: &Problem<V>,
    b: &mut GraphCoreBuilder<ConnWaitVertex, ConnWaitEdge>,
    mut tails: impl Iterator<Item = Transport>,
    mut heads: impl Iterator<Item = (Time, VIdx)>,
) -> Option<()> {
    let (mut due, mut head_v) = heads.next()?;
    let mut tail = tails.next()?;

    loop {
        let at = p.transport_by_idx(tail).destination().time();
        match at <= due {
            true => {
                // connect transport, and move to the next transport
                // due & head can still be used by the next transport
                b.edge(ConnWaitEdge::Exit(tail), t_into_v(tail), head_v);
                tail = tails.next()?;
            }
            false => {
                // couldn't connect transport, move to the next due & head
                // keep the transport as tail
                (due, head_v) = heads.next()?;
            }
        }
    }
}

fn t_into_v(t: Transport) -> VIdx {
    VIdx::from(t.into_inner())
}

fn conn_t_t<V: Variant>(
    p: &Problem<V>,
    b: &mut GraphCoreBuilder<ConnWaitVertex, ConnWaitEdge>,
    mut tails_rev: impl Iterator<Item = Transport>,
    mut heads_rev: Peekable<impl Iterator<Item = Transport>>,
) -> Option<()> {
    // no edges once we complete traversing heads
    let mut curr_head = heads_rev.next()?;

    // connect one tail per iteration
    loop {
        // no edges once we complete traversing tails
        let tail = tails_rev.next()?;

        match conn_t_t_find_head_for_tail(p, &mut heads_rev, curr_head, tail) {
            Some(head) => {
                b.edge(ConnWaitEdge::Connect(tail), t_into_v(tail), t_into_v(head));

                // same head can be assigned to prior tails
                curr_head = head;
            }
            // no head for this tail, moving on to the next tail
            None => {}
        }
    }
}

fn conn_t_t_find_head_for_tail<V: Variant>(
    p: &Problem<V>,
    heads_rev: &mut Peekable<impl Iterator<Item = Transport>>,
    curr_head: Transport,
    tail: Transport,
) -> Option<Transport> {
    // TODO: minor speed improvement possible by caching at of tail
    let feasible = |head: Transport| p.connectivity.can_connect_temporally(p, tail, head);

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

fn conn_ro_t<V: Variant>(
    p: &Problem<V>,
    b: &mut GraphCoreBuilder<ConnWaitVertex, ConnWaitEdge>,
    mut tails_rev: impl Iterator<Item = (Time, VIdx)>,
    mut heads_rev: Peekable<impl Iterator<Item = Transport>>,
) -> Option<()> {
    // no edges once we complete traversing heads
    let mut curr_head = heads_rev.next()?;

    // connect one tail per iteration
    loop {
        // no edges once we complete traversing tails
        let (tail_time, tail_v) = tails_rev.next()?;

        match conn_ro_t_find_head_for_tail(p, &mut heads_rev, curr_head, tail_time) {
            Some(head) => {
                b.edge(ConnWaitEdge::Enter, tail_v, t_into_v(head));

                // same head can be assigned to prior tails
                curr_head = head;
            }
            // no head for this tail, moving on to the next tail
            None => {}
        }
    }
}

fn conn_ro_t_find_head_for_tail<V: Variant>(
    p: &Problem<V>,
    heads_rev: &mut Peekable<impl Iterator<Item = Transport>>,
    curr_head: Transport,
    tail_time: Time,
) -> Option<Transport> {
    let feasible = |head: Transport| tail_time <= p.transport_by_idx(head).origin().time();

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
