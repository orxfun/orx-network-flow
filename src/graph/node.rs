use crate::graph::{in_edge::InEdge, out_edge::OutEdge};
use alloc::vec::Vec;

pub struct Node<N> {
    data: N,
    out_edges: Vec<OutEdge>,
    in_edges: Vec<InEdge>,
}

impl<N> Node<N> {
    pub fn new(data: N) -> Self {
        Self {
            data,
            out_edges: Vec::new(),
            in_edges: Vec::new(),
        }
    }

    pub fn out_edges(&self) -> &[OutEdge] {
        &self.out_edges
    }

    pub fn in_edges(&self) -> &[InEdge] {
        &self.in_edges
    }

    pub fn add_out_edge(&mut self, edges_idx: usize, head: usize, head_in_edge_idx: usize) {
        let out_edge = OutEdge::new(edges_idx, head, head_in_edge_idx);
        self.out_edges.push(out_edge);
    }

    pub fn add_in_edge(&mut self, edges_idx: usize, tail: usize, tail_out_edge_idx: usize) {
        let in_edge = InEdge::new(edges_idx, tail, tail_out_edge_idx);
        self.in_edges.push(in_edge);
    }
}
