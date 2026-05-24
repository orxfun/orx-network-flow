use crate::graph::{edge::Edge, node::Node};
use alloc::vec::Vec;

pub struct Graph<N, E> {
    pub(super) nodes: Vec<Node<N>>,
    pub(super) edges: Vec<Edge<E>>,
}
