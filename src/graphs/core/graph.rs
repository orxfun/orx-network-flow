use crate::graphs::core::{EdgeCore, GraphCore, VertexCore};
use crate::graphs::{EIdx, Graph, VIdx};

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
