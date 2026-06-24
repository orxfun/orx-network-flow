use crate::commodities::VecCommodity;
use crate::common_ds::SortedKeyMap;
use crate::graphs::{EIdx, EdgeRange, VIdx, core::GraphCore};
use crate::networks::GraphStats;
use crate::networks::aoa_wait_nw::visualization::dot::{AoaWaitDot, AoaWaitDotSettings};
use crate::networks::aoa_wait_nw::{AoaWaitEdge, AoaWaitVertex};
use crate::utils::std_utils::{Map, Set};
use crate::{Commodity, Problem, Space, SpaceTime, Time, Transport, Variant, VecTransport};

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
    pub fn stats(p: &Problem<V>, settings: AoaWaitNwSettings) -> GraphStats {
        let mut space_to_times: Map<Space, Set<Time>> = Default::default();
        let mut insert_st = |st: SpaceTime| {
            space_to_times
                .entry(st.space())
                .or_default()
                .insert(st.time());
        };

        for t in p.transports.indices() {
            let td = p.transport_by_idx(t);
            insert_st(td.origin());
            insert_st(td.destination());
        }

        for c in p.commodities.indices() {
            let com = p.commodity_by_idx(c);
            insert_st(com.origin());
            insert_st(com.destination());
        }

        let space_to_sorted_times = SortedKeyMap::from_sets_to_vecs(space_to_times);
        let mut st_to_v: Map<SpaceTime, usize> = Default::default();

        let mut next_v = 0usize;
        for (space, times) in space_to_sorted_times.iter() {
            for &time in times {
                let st = SpaceTime::new(*space, time);
                st_to_v.insert(st, next_v);
                next_v += 1;
            }
        }

        let mut num_edges = 0usize;

        let mut add_edge = |_tail: usize, _head: usize| {
            num_edges += 1;
        };

        // edges: wait arcs
        for (space, sorted_times) in space_to_sorted_times.iter() {
            let tails = sorted_times.iter().copied();
            let heads = sorted_times.iter().copied().skip(1);
            for (t1, t2) in tails.zip(heads) {
                let tail = st_to_v[&SpaceTime::new(*space, t1)];
                let head = st_to_v[&SpaceTime::new(*space, t2)];
                add_edge(tail, head);
            }
        }

        // edges: transport arcs
        for t in p.transports.indices() {
            let td = p.transport_by_idx(t);
            let tail = st_to_v[&td.origin()];
            let head = st_to_v[&td.destination()];
            add_edge(tail, head);
        }

        // edges: bypass arcs
        if settings.add_bypass_edges {
            for (_, com) in p.commodities.indices_values() {
                let tail = st_to_v[&com.origin()];
                let head = st_to_v[&com.destination()];
                add_edge(tail, head);
            }
        }

        GraphStats {
            num_vertices: next_v,
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

    pub fn as_dot_graph(&'a self, settings: Option<AoaWaitDotSettings>) -> AoaWaitDot<'a, V> {
        AoaWaitDot::new(self, settings)
    }
}
