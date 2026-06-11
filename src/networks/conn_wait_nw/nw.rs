use crate::graphs::core::GraphCore;
use crate::graphs::visualization::dot::NodeSettings;
use crate::networks::conn_wait_nw::visualization::dot::ConnWaitDot;
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitVertex};
use crate::{Problem, Variant};

pub type ConnWaitGraph = GraphCore<ConnWaitVertex, ConnWaitEdge>;

pub struct ConnWaitNw<'a, V>
where
    V: Variant,
{
    p: &'a Problem<V>,
    g: ConnWaitGraph,
}

impl<'a, V> ConnWaitNw<'a, V>
where
    V: Variant,
{
    pub fn p(&self) -> &Problem<V> {
        self.p
    }

    pub fn g(&self) -> &ConnWaitGraph {
        &self.g
    }

    pub fn as_dot_graph(&'a self, transport: Option<NodeSettings>) -> ConnWaitDot<'a, V> {
        ConnWaitDot::new(self, transport)
    }
}
