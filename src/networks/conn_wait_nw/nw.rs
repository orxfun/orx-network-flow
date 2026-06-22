use crate::commodities::VecCommodity;
use crate::graphs::{EIdx, EdgeRange, VIdx, core::GraphCore};
use crate::networks::conn_wait_nw::visualization::dot::{ConnWaitDot, ConnWaitDotSettings};
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitVertex};
use crate::networks::conn_wait_nw::{mcnf::solve, output::Output};
use crate::utils::std_utils::Map;
use crate::{Commodity, IdxCore, Problem, SpaceTime, Transport, Variant, VecTransport};
use alloc::vec::Vec;

pub struct ConnWaitNwSettings {
    pub add_bypass_edges: bool,
}

pub type ConnWaitGraph = GraphCore<ConnWaitVertex, ConnWaitEdge>;

pub struct ConnWaitNw<'a, V>
where
    V: Variant,
{
    pub(super) p: &'a Problem<V>,
    pub(super) g: ConnWaitGraph,
    pub(super) ro_to_v: Map<SpaceTime, VIdx>,
    pub(super) dd_to_v: Map<SpaceTime, VIdx>,
    pub(super) transport_edges: VecTransport<Vec<EIdx>>,
    pub(super) bypass_edges_range: EdgeRange,
    bypass_edge_per_commodity: VecCommodity<Option<EIdx>>,
}

// helpers
impl<V> ConnWaitNw<'_, V>
where
    V: Variant,
{
    pub(super) fn bypass_edge_of(&self, c: Commodity) -> EIdx {
        EIdx::from(self.bypass_edges_range.begin().into_inner() + c.into_inner())
    }

    pub(crate) fn p(&self) -> &Problem<V> {
        self.p
    }

    pub(crate) fn g(&self) -> &ConnWaitGraph {
        &self.g
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
impl<'a, V> ConnWaitNw<'a, V>
where
    V: Variant,
{
    pub fn construct(p: &'a Problem<V>, settings: ConnWaitNwSettings) -> Self {
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

    pub fn as_dot_graph(&'a self, settings: Option<ConnWaitDotSettings>) -> ConnWaitDot<'a, V> {
        ConnWaitDot::new(self, settings)
    }

    pub fn solve(&self, named: bool) -> Output<V> {
        solve(self, named)
    }
}
