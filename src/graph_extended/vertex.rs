use crate::VIdx;
use crate::graph::{InEdge, OutEdge};
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct ExtVertex<V, Ve> {
    core_vidx: VIdx,
    data: Ve,
    ext_out_edges: Vec<OutEdge>,
    ext_in_edges: Vec<InEdge>,
    core_data: PhantomData<V>,
}

impl<V, Ve> ExtVertex<V, Ve> {
    pub fn new(core_vidx: VIdx, data: Ve) -> Self {
        Self {
            core_vidx,
            data,
            ext_out_edges: Vec::new(),
            ext_in_edges: Vec::new(),
            core_data: PhantomData,
        }
    }
}
