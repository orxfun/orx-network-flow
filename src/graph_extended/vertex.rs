use crate::VIdx;
use crate::graph::{InEdge, OutEdge};
use alloc::vec::Vec;

pub struct ExtVertex<Ve> {
    core_vidx: VIdx,
    data: Ve,
    ext_out_edges: Vec<OutEdge>,
    ext_in_edges: Vec<InEdge>,
}
