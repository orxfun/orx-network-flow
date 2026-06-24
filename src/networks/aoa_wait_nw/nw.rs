use crate::commodities::VecCommodity;
use crate::graphs::{EIdx, EdgeRange, Graph, VIdx, core::GraphCore};
use crate::networks::GraphStats;
use crate::networks::aoa_wait_nw::visualization::dot::{AoaWaitDot, AoaWaitDotSettings};
use crate::networks::aoa_wait_nw::{AoaWaitEdge, AoaWaitVertex};
use crate::utils::std_utils::Map;
use crate::{Commodity, Problem, Space, SpaceTime, Time, Transport, Variant, VecTransport};

#[derive(Clone, Copy)]
pub struct AoaWaitNwSettings {
    pub add_bypass_edges: bool,
}

pub type AoaWaitGraph = GraphCore<AoaWaitVertex, AoaWaitEdge>;

pub struct AoaWaitNw<'a, V>
where
    V: Variant,
{
    p: &'a Problem<V>,
    g: AoaWaitGraph,
    /// Map from space-time pair to vertex index.
    st_to_v: Map<SpaceTime, VIdx>,
    /// Single arc index per transport.
    transport_arc: VecTransport<EIdx>,
    bypass_edges_range: EdgeRange,
    bypass_edge_per_commodity: VecCommodity<Option<EIdx>>,
}

// helpers
impl<V> AoaWaitNw<'_, V>
where
    V: Variant,
{
    pub(crate) fn p(&self) -> &Problem<V> {
        self.p
    }

    pub(crate) fn g(&self) -> &AoaWaitGraph {
        &self.g
    }

    pub(crate) fn bypass_edges_range(&self) -> EdgeRange {
        self.bypass_edges_range
    }

    pub(crate) fn transport_arcs(&self) -> impl Iterator<Item = (Transport, EIdx)> + '_ {
        self.transport_arc.enumerated_iter().map(|(t, &e)| (t, e))
    }

    pub(crate) fn bypass_edge_by_commodity(&self) -> &VecCommodity<Option<EIdx>> {
        &self.bypass_edge_per_commodity
    }

    pub(crate) fn bypass_edge_of(&self, c: Commodity) -> Option<EIdx> {
        self.bypass_edge_per_commodity[c]
    }

    pub(crate) fn st_to_v(&self) -> &Map<SpaceTime, VIdx> {
        &self.st_to_v
    }
}

// api
impl<'a, V> AoaWaitNw<'a, V>
where
    V: Variant,
{
    pub fn compute_stats(p: &Problem<V>, settings: AoaWaitNwSettings) -> GraphStats {
        let mut num_vertices = 0usize;
        let mut num_wait_edges = 0usize;
        for (space, _) in p.spaces.entries() {
            let num_times = count_unique_times_at_space(p, space);
            num_vertices += num_times;
            num_wait_edges += num_times.saturating_sub(1);
        }

        let mut num_edges = num_wait_edges + p.len_transports();
        if settings.add_bypass_edges {
            num_edges += p.len_commodities();
        }

        GraphStats {
            num_vertices,
            num_edges,
        }
    }

    pub fn construct(p: &'a Problem<V>, settings: AoaWaitNwSettings) -> Self {
        let output = super::construct::construct(p, settings);
        Self {
            p,
            g: output.graph,
            st_to_v: output.st_to_v,
            transport_arc: output.transport_arc,
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

    pub fn as_dot_graph(&'a self, settings: Option<AoaWaitDotSettings>) -> AoaWaitDot<'a, V> {
        AoaWaitDot::new(self, settings)
    }
}

fn count_unique_times_at_space<V: Variant>(p: &Problem<V>, space: Space) -> usize {
    let mut count = 0usize;

    // Source 1 (highest priority): origin times of transports that depart from `space`.
    if let Some(des_sorted_transports) = p.ori_des_sorted_transports.get(&space) {
        for (_, sorted_transports) in des_sorted_transports.iter() {
            for &t in sorted_transports {
                let time = p.transport_by_idx(t).origin().time();
                if !has_origin_transport_time_before(p, space, t, time) {
                    count += 1;
                }
            }
        }
    }

    // Source 2: destination times of transports that arrive to `space`.
    if let Some(ori_sorted_transports) = p.des_ori_sorted_transports.get(&space) {
        for (_, sorted_transports) in ori_sorted_transports.iter() {
            for &t in sorted_transports {
                let time = p.transport_by_idx(t).destination().time();
                if has_origin_transport_time(p, space, time) {
                    continue;
                }
                if !has_destination_transport_time_before(p, space, t, time) {
                    count += 1;
                }
            }
        }
    }

    // Source 3: ready times of origin commodities at `space`.
    if let Some(sorted_commodities) = p.ori_sorted_commodities.get(&space) {
        let mut prev_ready = None;
        for &c in sorted_commodities {
            let ready = p.commodity_by_idx(c).origin().time();
            if prev_ready == Some(ready) {
                continue;
            }
            prev_ready = Some(ready);

            if has_origin_transport_time(p, space, ready)
                || has_destination_transport_time(p, space, ready)
            {
                continue;
            }
            count += 1;
        }
    }

    // Source 4: due times of destination commodities at `space`.
    if let Some(sorted_commodities) = p.des_sorted_commodities.get(&space) {
        let mut prev_due = None;
        for &c in sorted_commodities {
            let due = p.commodity_by_idx(c).destination().time();
            if prev_due == Some(due) {
                continue;
            }
            prev_due = Some(due);

            if has_origin_transport_time(p, space, due)
                || has_destination_transport_time(p, space, due)
                || has_origin_commodity_time(p, space, due)
            {
                continue;
            }
            count += 1;
        }
    }

    count
}

fn has_origin_transport_time<V: Variant>(p: &Problem<V>, space: Space, time: Time) -> bool {
    if let Some(des_sorted_transports) = p.ori_des_sorted_transports.get(&space) {
        for (_, sorted_transports) in des_sorted_transports.iter() {
            for &t in sorted_transports {
                if p.transport_by_idx(t).origin().time() == time {
                    return true;
                }
            }
        }
    }
    false
}

fn has_destination_transport_time<V: Variant>(p: &Problem<V>, space: Space, time: Time) -> bool {
    if let Some(ori_sorted_transports) = p.des_ori_sorted_transports.get(&space) {
        for (_, sorted_transports) in ori_sorted_transports.iter() {
            for &t in sorted_transports {
                if p.transport_by_idx(t).destination().time() == time {
                    return true;
                }
            }
        }
    }
    false
}

fn has_origin_transport_time_before<V: Variant>(
    p: &Problem<V>,
    space: Space,
    current_t: Transport,
    time: Time,
) -> bool {
    if let Some(des_sorted_transports) = p.ori_des_sorted_transports.get(&space) {
        for (_, sorted_transports) in des_sorted_transports.iter() {
            for &t in sorted_transports {
                if t == current_t {
                    return false;
                }
                if p.transport_by_idx(t).origin().time() == time {
                    return true;
                }
            }
        }
    }
    false
}

fn has_destination_transport_time_before<V: Variant>(
    p: &Problem<V>,
    space: Space,
    current_t: Transport,
    time: Time,
) -> bool {
    if let Some(ori_sorted_transports) = p.des_ori_sorted_transports.get(&space) {
        for (_, sorted_transports) in ori_sorted_transports.iter() {
            for &t in sorted_transports {
                if t == current_t {
                    return false;
                }
                if p.transport_by_idx(t).destination().time() == time {
                    return true;
                }
            }
        }
    }
    false
}

fn has_origin_commodity_time<V: Variant>(p: &Problem<V>, space: Space, time: Time) -> bool {
    if let Some(sorted_commodities) = p.ori_sorted_commodities.get(&space) {
        for &c in sorted_commodities {
            if p.commodity_by_idx(c).origin().time() == time {
                return true;
            }
        }
    }
    false
}
