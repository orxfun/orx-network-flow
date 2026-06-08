use crate::graphs::{EIdx, Edge, VIdx, Vertex};

pub trait Graph {
    type Dv;

    type De;

    type V<'a>: Vertex<Data = Self::Dv>
    where
        Self: 'a;

    type E<'a>: Edge<Data = Self::De>
    where
        Self: 'a;

    fn v(&self) -> usize;

    fn e(&self) -> usize;

    fn vertices(&self) -> impl Iterator<Item = Self::V<'_>>;

    fn edges(&self) -> impl Iterator<Item = Self::E<'_>>;

    fn vertex_indices(&self) -> impl ExactSizeIterator<Item = VIdx> {
        (0..self.v()).map(VIdx::from)
    }

    fn edge_indices(&self) -> impl ExactSizeIterator<Item = EIdx> {
        (0..self.e()).map(EIdx::from)
    }

    fn vertex(&self, v: VIdx) -> Self::V<'_>;

    fn edge(&self, e: EIdx) -> Self::E<'_>;
}
