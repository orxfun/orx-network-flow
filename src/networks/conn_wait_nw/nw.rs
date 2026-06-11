use crate::graphs::core::GraphCore;
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
