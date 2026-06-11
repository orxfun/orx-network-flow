use crate::networks::conn_wait_nw::ConnWaitGraph;
use crate::{Problem, Variant};

pub fn construct_graph<V: Variant>(p: &Problem<V>) -> ConnWaitGraph {
    let mut builder = ConnWaitGraph::builder();
    let b = &mut builder;

    builder.finish()
}
