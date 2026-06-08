use crate::graphs::{EIdx, Edge, VIdx, Vertex};

pub trait Graph {
    type V<'a>: Vertex
    where
        Self: 'a;

    type E<'a>: Edge
    where
        Self: 'a;

    fn vertex(&self, v: VIdx) -> Self::V<'_>;

    fn edge(&self, e: EIdx) -> Self::E<'_>;
}
