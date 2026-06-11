use crate::graphs::{
    EIdx, Edge, VIdx, Vertex,
    visualization::dot::{DotGraphBasic, VertexSettings},
};

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

    fn vertex<'a>(&'a self, v: VIdx) -> Self::V<'a>
    where
        Self: 'a;

    fn edge<'a>(&'a self, e: EIdx) -> Self::E<'a>
    where
        Self: 'a;

    // visualization

    fn as_basic_dot_graph(&self, custom_settings: Option<VertexSettings>) -> DotGraphBasic<'_, Self>
    where
        Self: Sized,
    {
        DotGraphBasic::new_with_settings(self, custom_settings.unwrap_or_default())
    }
}
