use crate::graph::{graph::Graph, node::Node};
use alloc::vec::Vec;

pub struct GraphBuilder<N, E>(Graph<N, E>);

impl<N, E> GraphBuilder<N, E> {
    pub fn new(num_nodes: usize, data: impl Fn(usize) -> N) -> Self {
        let nodes: Vec<_> = (0..num_nodes).map(data).map(Node::new).collect();
        let edges = Vec::new();
        let graph = Graph { nodes, edges };
        Self(graph)
    }
}
