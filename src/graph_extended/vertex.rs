use crate::VIdx;
use crate::graph::{InEdge, OutEdge};
use alloc::vec::Vec;

pub struct ExtVertex {
    core_vidx: VIdx,
    ext_out_edges: Vec<OutEdge>,
    ext_in_edges: Vec<InEdge>,
}
