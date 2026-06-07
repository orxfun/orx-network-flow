use crate::VIdx;
use crate::graph::{InEdge, OutEdge, Vertex};
use alloc::vec::Vec;

pub enum ExtVertex<V, Ve> {
    Core(CoreVertex<V>),
    Ext(Vertex<Ve>),
}

pub struct CoreVertex<Ve> {
    core_vidx: VIdx,
    data: Ve,
    ext_out_edges: Vec<OutEdge>,
    ext_in_edges: Vec<InEdge>,
}

impl<Ve> CoreVertex<Ve> {
    pub fn new(core_vidx: VIdx, data: Ve) -> Self {
        Self {
            core_vidx,
            data,
            ext_out_edges: Vec::new(),
            ext_in_edges: Vec::new(),
        }
    }
}
