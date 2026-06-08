use crate::graphs::{EIdx, Edge, VIdx, Vertex};

pub trait Graph {
    type V: Vertex;

    type E: Edge;

    fn v(&self) -> usize {
        self.vertices().len()
    }

    fn e(&self) -> usize {
        self.edges().len()
    }

    fn vertices(&self) -> impl ExactSizeIterator<Item = &Self::V>;

    fn edges(&self) -> impl ExactSizeIterator<Item = &Self::E>;

    fn vertex_indices(&self) -> impl ExactSizeIterator<Item = VIdx> {
        (0..self.v()).map(VIdx::from)
    }

    fn edge_indices(&self) -> impl ExactSizeIterator<Item = EIdx> {
        (0..self.e()).map(EIdx::from)
    }

    fn vertex(&self, v: VIdx) -> &Self::V;

    fn edge(&self, e: EIdx) -> &Self::E;
}

pub type VertexDataOf<G> = <<G as Graph>::V as Vertex>::Data;

pub type EdgeDataOf<G> = <<G as Graph>::V as Edge>::Data;
