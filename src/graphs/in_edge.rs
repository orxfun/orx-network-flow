use crate::graphs::{EIdx, VIdx};

pub struct InEdge {
    edges_idx: EIdx,
    tail: VIdx,
    tail_out_edge_pos: usize,
}

impl InEdge {
    pub fn new(edges_idx: EIdx, tail: VIdx, tail_out_edge_pos: usize) -> Self {
        Self {
            edges_idx,
            tail,
            tail_out_edge_pos,
        }
    }

    pub fn edges_idx(&self) -> EIdx {
        self.edges_idx
    }

    pub fn tail(&self) -> VIdx {
        self.tail
    }

    pub fn tail_out_edge_idx(&self) -> usize {
        self.tail_out_edge_pos
    }
}
