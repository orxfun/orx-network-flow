use crate::graphs::core::{EdgeCore, GraphCore, VertexCore};
use crate::graphs::{EIdx, Graph, VIdx};

impl<Dv, De> Graph for GraphCore<Dv, De> {
    type Dv = Dv;

    type De = De;

    type V<'a>
        = &'a VertexCore<Dv>
    where
        Self: 'a;

    type E<'a>
        = &'a EdgeCore<De>
    where
        Self: 'a;

    fn v(&self) -> usize {
        self.vertices.len()
    }

    fn e(&self) -> usize {
        self.edges.len()
    }

    fn vertices(&self) -> impl Iterator<Item = Self::V<'_>> {
        self.vertices.iter()
    }

    fn edges(&self) -> impl Iterator<Item = Self::E<'_>> {
        self.edges.iter()
    }

    fn vertex(&self, v: VIdx) -> Self::V<'_> {
        &self.vertices[v]
    }

    fn edge(&self, e: EIdx) -> Self::E<'_> {
        &self.edges[e]
    }
}
