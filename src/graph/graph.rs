use crate::graph::{edge::Edge, node::Node};
use alloc::vec::Vec;

pub struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}
