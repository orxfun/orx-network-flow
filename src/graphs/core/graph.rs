use crate::graphs::core::{EdgeCore, GraphCore, VertexCore};
use crate::graphs::{EIdx, Graph, VIdx};

impl<Dv, De> Graph for GraphCore<Dv, De> {
    type V = VertexCore<Dv>;

    type E = EdgeCore<De>;

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
