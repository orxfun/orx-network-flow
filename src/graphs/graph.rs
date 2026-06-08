use crate::graphs::{Edge, Vertex, edge::EIdx, vertex::VIdx};

pub trait Graph {
    type V;

    type E;

    fn vertex(&self, v: VIdx) -> &Vertex<Self::V>;

    fn edge(&self, e: EIdx) -> &Edge<Self::E>;
}
