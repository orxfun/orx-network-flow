use crate::graphs::{EIdx, VIdx, Vertex, core::VertexCore};
use alloc::vec::Vec;

pub struct OriVertex<V> {
    pub(super) core_v: VIdx,
    pub(super) ext_data: V,
    pub(super) ext_out_edges: Vec<EIdx>,
    pub(super) ext_in_edges: Vec<EIdx>,
}

pub enum ExtVertex<V> {
    Ori(OriVertex<V>),
    New(VertexCore<V>),
}

impl<V> Vertex for ExtVertex<V> {
    type Data = V;

    fn data(&self) -> &Self::Data {
        match self {
            Self::Ori(o) => &o.ext_data,
            Self::New(v) => v.data(),
        }
    }

    fn out_edges(&self) -> impl ExactSizeIterator<Item = EIdx> {
        match self {
            Self::Ori(o) => todo!(),
            Self::New(v) => v.out_edges(),
        }
    }

    fn in_edges(&self) -> impl ExactSizeIterator<Item = EIdx> {
        core::iter::empty()
    }
}
