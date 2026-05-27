use crate::graph::{builder::GraphBuilder, edge::Edge, vertex::Vertex};
use alloc::vec::Vec;

pub struct Graph<V, E> {
    pub(super) vertices: Vec<Vertex<V>>,
    pub(super) edges: Vec<Edge<E>>,
}

impl<V, E> Graph<V, E> {
    pub fn builder(vertices: impl Iterator<Item = V>) -> GraphBuilder<V, E> {
        GraphBuilder::new(vertices)
    }
}
