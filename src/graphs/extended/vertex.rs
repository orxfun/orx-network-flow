use crate::graphs::{InEdge, OutEdge, VIdx};
use alloc::vec::Vec;

pub struct OriVertex<V> {
    pub(super) core_v: VIdx,
    pub(super) ext_data: V,
    pub(super) ext_out_edges: Vec<OutEdge>,
    pub(super) ext_in_edges: Vec<InEdge>,
}
