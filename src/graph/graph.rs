use crate::VIdx;
use crate::graph::edge::VecEdge;
use crate::graph::vertex::{VecVertex, Vertex};
use crate::graph::{builder::GraphBuilder, edge::Edge};

pub struct Graph<V, E> {
    pub(super) vertices: VecVertex<Vertex<V>>,
    pub(super) edges: VecEdge<Edge<E>>,
}

impl<V, E> Graph<V, E> {
    pub fn builder(vertices: impl Iterator<Item = V>) -> GraphBuilder<V, E> {
        GraphBuilder::new(vertices)
    }

    pub fn len_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn len_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn vertex(&self, vidx: VIdx) -> &Vertex<V> {
        &self.vertices[vidx]
    }
}
