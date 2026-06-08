use crate::graphs::EIdx;
use crate::graphs::graph::Graph;
use crate::graphs::graph_core::EdgeCore;
use crate::graphs::{GraphBuilder, VIdx, VecEdge, VecVertex, graph_core::vertex::VertexCore};

pub struct GraphCore<V, E> {
    pub(super) vertices: VecVertex<VertexCore<V>>,
    pub(super) edges: VecEdge<EdgeCore<E>>,
}

impl<V, E> GraphCore<V, E> {
    pub fn builder(vertices: impl Iterator<Item = V>) -> GraphBuilder<V, E> {
        GraphBuilder::new(vertices)
    }

    pub fn len_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn len_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn vertex(&self, vidx: VIdx) -> &VertexCore<V> {
        &self.vertices[vidx]
    }
}

impl<V, E> Graph for GraphCore<V, E> {
    type V<'a>
        = &'a VertexCore<V>
    where
        Self: 'a;

    type E<'a>
        = &'a EdgeCore<E>
    where
        Self: 'a;

    fn vertices(&self) -> impl ExactSizeIterator<Item = Self::V<'_>> {
        self.vertices.iter()
    }

    fn edges(&self) -> impl ExactSizeIterator<Item = Self::E<'_>> {
        self.edges.iter()
    }

    fn vertex(&self, v: VIdx) -> Self::V<'_> {
        &self.vertices[v]
    }

    fn edge(&self, e: EIdx) -> Self::E<'_> {
        &self.edges[e]
    }
}
