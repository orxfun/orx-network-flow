use crate::graphs::{EIdx, InEdge, OutEdge, VIdx, Vertex, core::VertexCore};
use alloc::vec::Vec;

pub struct OriVertex<V> {
    pub(super) core_v: VIdx,
    pub(super) ext_data: V,
    pub(super) ext_out_edges: Vec<OutEdge>,
    pub(super) ext_in_edges: Vec<InEdge>,
}

pub enum ExtVertex<V> {
    Ori(OriVertex<V>),
    New(VertexCore<V>),
}

impl<V> Vertex for ExtVertex<V> {
    type Data = ();

    fn data(&self) -> &Self::Data {
        todo!()
    }

    fn out_edges(&self) -> impl ExactSizeIterator<Item = EIdx> {
        core::iter::empty()
    }

    fn in_edges(&self) -> impl ExactSizeIterator<Item = EIdx> {
        core::iter::empty()
    }
}
