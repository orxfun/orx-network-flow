use crate::graph::{edge::EIdx, vertex::VIdx};

pub struct OutEdge {
    edges_idx: EIdx,
    head: VIdx,
    head_in_edge_pos: usize,
}

impl OutEdge {
    pub fn new(edges_idx: EIdx, head: VIdx, head_in_edge_pos: usize) -> Self {
        Self {
            edges_idx,
            head,
            head_in_edge_pos,
        }
    }

    pub fn edges_idx(&self) -> EIdx {
        self.edges_idx
    }

    pub fn head(&self) -> VIdx {
        self.head
    }

    pub fn head_in_edge_idx(&self) -> usize {
        self.head_in_edge_pos
    }
}
