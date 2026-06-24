use crate::commodities::VecCommodity;
use crate::graphs::{EIdx, EdgeRange, Graph, VIdx, core::GraphCore};
use crate::networks::GraphStats;
use crate::networks::aon_wait_nw::visualization::dot::{AonWaitDot, AonWaitDotSettings};
use crate::networks::aon_wait_nw::{AonWaitEdge, AonWaitVertex};
use crate::networks::aon_wait_nw::{mcnf::solve, output::Output};
use crate::utils::std_utils::Map;
use crate::{Commodity, IdxCore, Problem, SpaceTime, Transport, Variant, VecTransport};
use alloc::vec::Vec;

#[derive(Clone, Copy)]
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
    pub fn compute_stats(p: &Problem<V>, settings: AonWaitNwSettings) -> GraphStats {
        // transport vertices + unique (origin, ready_time) + unique (destination, due_time)
        let mut num_vertices = p.len_transports();

        for (ori, sorted_commodities) in p.ori_sorted_commodities.iter() {
            let mut prev_ready = None;
            for &c in sorted_commodities {
                let com = p.commodity_by_idx(c);
                debug_assert_eq!(*ori, com.origin().space());
                let ready = com.origin().time();
                if prev_ready != Some(ready) {
                    num_vertices += 1;
                    prev_ready = Some(ready);
                }
            }
        }

        for (des, sorted_commodities) in p.des_sorted_commodities.iter() {
            let mut prev_due = None;
            for &c in sorted_commodities {
                let com = p.commodity_by_idx(c);
                debug_assert_eq!(*des, com.destination().space());
                let due = com.destination().time();
                if prev_due != Some(due) {
                    num_vertices += 1;
                    prev_due = Some(due);
                }
            }
        }

        let mut num_edges = 0usize;

        // edges: t-t wait
        for (_, des_transports) in p.ori_des_sorted_transports.iter() {
            for (_, transports) in des_transports.iter() {
                num_edges += transports.len().saturating_sub(1);
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

                            num_edges += 1;
                        }
                    }
                }
            }
        }

        // edges: ro-t connect
        for (&ori, sorted_commodities) in p.ori_sorted_commodities.iter() {
            if let Some(des_sorted_transports) = p.ori_des_sorted_transports.get(&ori) {
                for (_, sorted_transports) in des_sorted_transports.iter() {
                    if sorted_transports.is_empty() {
                        continue;
                    }

                    let mut curr_head_idx = sorted_transports.len() - 1;
                    let mut prev_ready = None;
                    for &c in sorted_commodities.iter().rev() {
                        let ready = p.commodity_by_idx(c).origin().time();
                        if prev_ready == Some(ready) {
                            continue;
                        }
                        prev_ready = Some(ready);

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

                        num_edges += 1;
                    }
                }
            }
        }

        // edges: t-dd connect
        for (&des, sorted_commodities) in p.des_sorted_commodities.iter() {
            if let Some(ori_sorted_transports) = p.des_ori_sorted_transports.get(&des) {
                for (_, sorted_transports) in ori_sorted_transports.iter() {
                    if sorted_transports.is_empty() || sorted_commodities.is_empty() {
                        continue;
                    }

                    let mut tail_idx = 0usize;
                    let mut due_idx = 0usize;

                    while tail_idx < sorted_transports.len() && due_idx < sorted_commodities.len() {
                        let tail = sorted_transports[tail_idx];
                        let due_commodity = sorted_commodities[due_idx];
                        let due = p.commodity_by_idx(due_commodity).destination().time();
                        let at = p.transport_by_idx(tail).destination().time();

                        if at <= due {
                            num_edges += 1;
                            tail_idx += 1;
                        } else {
                            due_idx += 1;
                            while due_idx < sorted_commodities.len() {
                                let next_due = p
                                    .commodity_by_idx(sorted_commodities[due_idx])
                                    .destination()
                                    .time();
                                if next_due != due {
                                    break;
                                }
                                due_idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // edges: ro-dd bypass
        if settings.add_bypass_edges {
            num_edges += p.len_commodities();
        }

        GraphStats {
            num_vertices,
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

    pub fn stats(&self) -> GraphStats {
        GraphStats {
            num_vertices: self.g.v(),
            num_edges: self.g.e(),
        }
    }

    pub fn as_dot_graph(&'a self, settings: Option<AonWaitDotSettings>) -> AonWaitDot<'a, V> {
        AonWaitDot::new(self, settings)
    }

    pub fn solve(&self, named: bool) -> Output<V> {
        solve(self, named)
    }
}
