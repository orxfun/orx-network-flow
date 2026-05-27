use crate::graph::vertex::{VecVertex, Vertex};
use crate::graph::{builder::GraphBuilder, edge::Edge};
use alloc::vec::Vec;

pub struct Graph<V, E> {
    pub(super) vertices: VecVertex<Vertex<V>>,
    pub(super) edges: Vec<Edge<E>>,
}

impl<V, E> Graph<V, E> {
    pub fn builder(vertices: impl Iterator<Item = V>) -> GraphBuilder<V, E> {
        GraphBuilder::new(vertices)
    }
}
