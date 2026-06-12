use crate::graphs::VIdx;
use crate::graphs::core::GraphCore;
use crate::networks::conn_wait_nw::visualization::dot::{ConnWaitDot, ConnWaitDotSettings};
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitVertex};
use crate::utils::std_utils::Map;
use crate::{Problem, SpaceTime, Variant};

pub struct ConnWaitNwSettings {
    pub add_bypass_edges: bool,
}

pub type ConnWaitGraph = GraphCore<ConnWaitVertex, ConnWaitEdge>;

pub struct ConnWaitNw<'a, V>
where
    V: Variant,
{
    p: &'a Problem<V>,
    g: ConnWaitGraph,
    ro_to_v: Map<SpaceTime, VIdx>,
}

impl<'a, V> ConnWaitNw<'a, V>
where
    V: Variant,
{
    pub fn construct(p: &'a Problem<V>, settings: ConnWaitNwSettings) -> Self {
        let g = super::construct::construct_graph(p, settings);
        Self { p, g }
    }

    pub fn p(&self) -> &Problem<V> {
        self.p
    }

    pub fn g(&self) -> &ConnWaitGraph {
        &self.g
    }

    pub fn as_dot_graph(&'a self, settings: Option<ConnWaitDotSettings>) -> ConnWaitDot<'a, V> {
        ConnWaitDot::new(self, settings)
    }
}
