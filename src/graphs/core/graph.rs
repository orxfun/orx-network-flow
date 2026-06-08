use crate::graphs::core::{EdgeCore, GraphCore, VertexCore};
use crate::graphs::{EIdx, Graph, VIdx};

impl<V, E> Graph for GraphCore<V, E> {
    type V = VertexCore<V>;

    type E = EdgeCore<E>;

    fn vertices(&self) -> impl ExactSizeIterator<Item = &Self::V> {
        self.vertices.iter()
    }

    fn edges(&self) -> impl ExactSizeIterator<Item = &Self::E> {
        self.edges.iter()
    }

    fn vertex(&self, v: VIdx) -> &Self::V {
        &self.vertices[v]
    }

    fn edge(&self, e: EIdx) -> &Self::E {
        &self.edges[e]
    }
}
