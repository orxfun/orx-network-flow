use crate::graphs::{EIdx, Edge, VIdx, Vertex};

pub trait Graph {
    type V<'a>: Vertex
    where
        Self: 'a;

    type E<'a>: Edge
    where
        Self: 'a;

    fn v(&self) -> usize {
        self.vertices().len()
    }

    fn e(&self) -> usize {
        self.edges().len()
    }

    fn vertices(&self) -> impl ExactSizeIterator<Item = Self::V<'_>>;

    fn edges(&self) -> impl ExactSizeIterator<Item = Self::E<'_>>;

    fn vertex_indices(&self) -> impl ExactSizeIterator<Item = VIdx> {
        (0..self.v()).map(VIdx::from)
    }

    fn edge_indices(&self) -> impl ExactSizeIterator<Item = EIdx> {
        (0..self.e()).map(EIdx::from)
    }

    fn vertex(&self, v: VIdx) -> Self::V<'_>;

    fn edge(&self, e: EIdx) -> Self::E<'_>;
}
