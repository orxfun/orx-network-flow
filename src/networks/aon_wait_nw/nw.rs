use crate::commodities::VecCommodity;
use crate::common_ds::SortedKeyMap;
use crate::graphs::{EIdx, EdgeRange, VIdx, core::GraphCore};
use crate::networks::GraphStats;
use crate::networks::aon_wait_nw::visualization::dot::{AonWaitDot, AonWaitDotSettings};
use crate::networks::aon_wait_nw::{AonWaitEdge, AonWaitVertex};
use crate::networks::aon_wait_nw::{mcnf::solve, output::Output};
use crate::utils::std_utils::{Map, Set};
use crate::{
    Commodity, IdxCore, Problem, Space, SpaceTime, Time, Transport, Variant, VecTransport,
};
use alloc::vec::Vec;

pub struct AonWaitNwSettings {
    pub add_bypass_edges: bool,
}

pub type AonWaitGraph = GraphCore<AonWaitVertex, AonWaitEdge>;

pub struct AonWaitNw<'a, V>
where
    V: Variant,
{
    p: &'a Problem<V>,
    g: AonWaitGraph,
    ro_to_v: Map<SpaceTime, VIdx>,
    dd_to_v: Map<SpaceTime, VIdx>,
    transport_edges: VecTransport<Vec<EIdx>>,
    bypass_edges_range: EdgeRange,
    bypass_edge_per_commodity: VecCommodity<Option<EIdx>>,
}

// helpers
impl<V> AonWaitNw<'_, V>
where
    V: Variant,
{
    pub(super) fn bypass_edge_of(&self, c: Commodity) -> EIdx {
        EIdx::from(self.bypass_edges_range.begin().into_inner() + c.into_inner())
    }

    pub(crate) fn p(&self) -> &Problem<V> {
        &self.p
    }

    pub(crate) fn g(&self) -> &AonWaitGraph {
        &self.g
    }

    pub(crate) fn ro_to_v(&self) -> &Map<SpaceTime, VIdx> {
        &self.ro_to_v
    }

    pub(crate) fn dd_to_v(&self) -> &Map<SpaceTime, VIdx> {
        &self.dd_to_v
    }

    pub(crate) fn bypass_edges_range(&self) -> EdgeRange {
        self.bypass_edges_range
    }

    pub(crate) fn transport_edges(&self) -> impl Iterator<Item = (Transport, &[EIdx])> {
        self.transport_edges
            .enumerated_iter()
            .map(|(a, b)| (a, b.as_slice()))
    }

    pub(crate) fn bypass_edge_by_commodity(&self) -> &VecCommodity<Option<EIdx>> {
        &self.bypass_edge_per_commodity
    }
}

// api
impl<'a, V> AonWaitNw<'a, V>
where
    V: Variant,
{
    pub fn stats(p: &Problem<V>, settings: AonWaitNwSettings) -> GraphStats {
        let mut next_v = p.len_transports();

        let mut ro_to_v: Map<SpaceTime, usize> = Default::default();
        let mut ori_to_sorted_ready: Map<Space, Set<Time>> = Default::default();
        for (ori, sorted_commodities) in p.ori_sorted_commodities.iter() {
            for &c in sorted_commodities {
                let com = p.commodity_by_idx(c);
                debug_assert_eq!(*ori, com.origin().space());

                ori_to_sorted_ready
                    .entry(*ori)
                    .or_default()
                    .insert(com.origin().time());

                let ro = com.origin();
                if !ro_to_v.contains_key(&ro) {
                    ro_to_v.insert(ro, next_v);
                    next_v += 1;
                }
            }
        }
        let ori_to_sorted_ready = SortedKeyMap::from_sets_to_vecs(ori_to_sorted_ready);

        let mut dd_to_v: Map<SpaceTime, usize> = Default::default();
        let mut des_to_sorted_due: Map<Space, Set<Time>> = Default::default();
        for (des, sorted_commodities) in p.des_sorted_commodities.iter() {
            for &c in sorted_commodities {
                let com = p.commodity_by_idx(c);
                debug_assert_eq!(*des, com.destination().space());

                des_to_sorted_due
                    .entry(*des)
                    .or_default()
                    .insert(com.destination().time());

                let dd = com.destination();
                if !dd_to_v.contains_key(&dd) {
                    dd_to_v.insert(dd, next_v);
                    next_v += 1;
                }
            }
        }
        let des_to_sorted_due = SortedKeyMap::from_sets_to_vecs(des_to_sorted_due);

        let mut num_edges = 0usize;

        let mut add_edge = |_tail: usize, _head: usize| {
            num_edges += 1;
        };

        let t_into_v = |t: Transport| t.into_inner();

        // edges: t-t wait
        for (_, des_transports) in p.ori_des_sorted_transports.iter() {
            for (_, transports) in des_transports.iter() {
                let tails = transports.iter().copied();
                let heads = transports.iter().copied().skip(1);
                for (tail, head) in tails.zip(heads) {
                    add_edge(t_into_v(tail), t_into_v(head));
                }
            }
        }

        // edges: t-t connect
        for (x, des_sorted_transports) in p.ori_des_sorted_transports.iter() {
            for (des, tail_sorted_transports) in des_sorted_transports.iter() {
                if let Some(map_head_sorted_transports) = p.ori_des_sorted_transports.get(des) {
                    for (y, head_sorted_transports) in map_head_sorted_transports.iter() {
                        if !p.connectivity.can_connect_spatially(p, [*x, *des, *y]) {
                            continue;
                        }

                        if head_sorted_transports.is_empty() {
                            continue;
                        }

                        let mut curr_head_idx = head_sorted_transports.len() - 1;
                        for &tail in tail_sorted_transports.iter().rev() {
                            let mut curr_head = head_sorted_transports[curr_head_idx];
                            let feasible = |head: Transport| {
                                p.connectivity.can_connect_temporally(p, tail, head)
                            };

                            if !feasible(curr_head) {
                                continue;
                            }

                            while curr_head_idx > 0 {
                                let next_idx = curr_head_idx - 1;
                                let next_head = head_sorted_transports[next_idx];
                                if feasible(next_head) {
                                    curr_head_idx = next_idx;
                                    curr_head = next_head;
                                } else {
                                    break;
                                }
                            }

                            add_edge(t_into_v(tail), t_into_v(curr_head));
                        }
                    }
                }
            }
        }

        // edges: ro-t connect
        for (&ori, sorted_ready) in ori_to_sorted_ready.iter() {
            if let Some(des_sorted_transports) = p.ori_des_sorted_transports.get(&ori) {
                for (_, sorted_transports) in des_sorted_transports.iter() {
                    if sorted_transports.is_empty() {
                        continue;
                    }

                    let mut curr_head_idx = sorted_transports.len() - 1;
                    for &ready in sorted_ready.iter().rev() {
                        let mut curr_head = sorted_transports[curr_head_idx];
                        let feasible =
                            |head: Transport| ready <= p.transport_by_idx(head).origin().time();

                        if !feasible(curr_head) {
                            continue;
                        }

                        while curr_head_idx > 0 {
                            let next_idx = curr_head_idx - 1;
                            let next_head = sorted_transports[next_idx];
                            if feasible(next_head) {
                                curr_head_idx = next_idx;
                                curr_head = next_head;
                            } else {
                                break;
                            }
                        }

                        let ro = SpaceTime::new(ori, ready);
                        let tail = ro_to_v[&ro];
                        add_edge(tail, t_into_v(curr_head));
                    }
                }
            }
        }

        // edges: t-dd connect
        for (&des, sorted_due) in des_to_sorted_due.iter() {
            if let Some(ori_sorted_transports) = p.des_ori_sorted_transports.get(&des) {
                for (_, sorted_transports) in ori_sorted_transports.iter() {
                    if sorted_transports.is_empty() || sorted_due.is_empty() {
                        continue;
                    }

                    let mut tail_idx = 0usize;
                    let mut due_idx = 0usize;

                    while tail_idx < sorted_transports.len() && due_idx < sorted_due.len() {
                        let tail = sorted_transports[tail_idx];
                        let due = sorted_due[due_idx];
                        let at = p.transport_by_idx(tail).destination().time();

                        if at <= due {
                            let dd = SpaceTime::new(des, due);
                            let head = dd_to_v[&dd];
                            add_edge(t_into_v(tail), head);
                            tail_idx += 1;
                        } else {
                            due_idx += 1;
                        }
                    }
                }
            }
        }

        // edges: ro-dd bypass
        if settings.add_bypass_edges {
            for (_, com) in p.commodities.indices_values() {
                let ro = ro_to_v[&com.origin()];
                let dd = dd_to_v[&com.destination()];
                add_edge(ro, dd);
            }
        }

        GraphStats {
            num_vertices: next_v,
            num_edges,
        }
    }

    pub fn construct(p: &'a Problem<V>, settings: AonWaitNwSettings) -> Self {
        let output = super::construct::construct(p, settings);
        Self {
            p,
            g: output.graph,
            ro_to_v: output.ro_to_v,
            dd_to_v: output.dd_to_v,
            transport_edges: output.transport_edges,
            bypass_edges_range: output.bypass_edges_range,
            bypass_edge_per_commodity: output.bypass_edge_per_commodity,
        }
    }

    pub fn as_dot_graph(&'a self, settings: Option<AonWaitDotSettings>) -> AonWaitDot<'a, V> {
        AonWaitDot::new(self, settings)
    }

    pub fn solve(&self, named: bool) -> Output<V> {
        solve(self, named)
    }
}
