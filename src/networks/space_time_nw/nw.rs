use crate::commodities::VecCommodity;
use crate::graphs::{EIdx, EdgeRange, VIdx, core::GraphCore};
use crate::networks::space_time_nw::visualization::dot::{SpaceTimeDot, SpaceTimeDotSettings};
use crate::networks::space_time_nw::{SpaceTimeEdge, SpaceTimeVertex};
use crate::utils::std_utils::Map;
use crate::{Commodity, Problem, SpaceTime, Transport, Variant, VecTransport};

pub struct SpaceTimeNwSettings {
    pub add_bypass_edges: bool,
}

pub type SpaceTimeGraph = GraphCore<SpaceTimeVertex, SpaceTimeEdge>;

pub struct SpaceTimeNw<'a, V>
where
    V: Variant,
{
    p: &'a Problem<V>,
    g: SpaceTimeGraph,
    /// Map from space-time pair to vertex index.
    st_to_v: Map<SpaceTime, VIdx>,
    /// Single arc index per transport.
    transport_arc: VecTransport<EIdx>,
    bypass_edges_range: EdgeRange,
    bypass_edge_per_commodity: VecCommodity<Option<EIdx>>,
}

// helpers
impl<V> SpaceTimeNw<'_, V>
where
    V: Variant,
{
    pub(crate) fn p(&self) -> &Problem<V> {
        self.p
    }

    pub(crate) fn g(&self) -> &SpaceTimeGraph {
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
impl<'a, V> SpaceTimeNw<'a, V>
where
    V: Variant,
{
    pub fn construct(p: &'a Problem<V>, settings: SpaceTimeNwSettings) -> Self {
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

    pub fn as_dot_graph(&'a self, settings: Option<SpaceTimeDotSettings>) -> SpaceTimeDot<'a, V> {
        SpaceTimeDot::new(self, settings)
    }
}
