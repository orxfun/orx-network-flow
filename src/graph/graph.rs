use crate::graph::{builder::GraphBuilder, edge::Edge, node::Node};
use alloc::vec::Vec;

pub struct Graph<N, E> {
    pub(super) nodes: Vec<Node<N>>,
    pub(super) edges: Vec<Edge<E>>,
}

impl<N, E> Graph<N, E> {
    pub fn builder(num_nodes: usize, data: impl Fn(usize) -> N) -> GraphBuilder<N, E> {
        GraphBuilder::new(num_nodes, data)
    }
}
