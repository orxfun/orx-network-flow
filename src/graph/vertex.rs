use crate::graph::{edge::EIdx, in_edge::InEdge, out_edge::OutEdge};
use alloc::vec::Vec;
use core::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VIdx(pub(super) usize);

impl Display for VIdx {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

pub struct Vertex<V> {
    data: V,
    out_edges: Vec<OutEdge>,
    in_edges: Vec<InEdge>,
}

impl<N> Vertex<N> {
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

    pub fn add_out_edge(&mut self, edges_idx: EIdx, head: VIdx, head_in_edge_idx: usize) {
        let out_edge = OutEdge::new(edges_idx, head, head_in_edge_idx);
        self.out_edges.push(out_edge);
    }

    pub fn add_in_edge(&mut self, edges_idx: EIdx, tail: VIdx, tail_out_edge_idx: usize) {
        let in_edge = InEdge::new(edges_idx, tail, tail_out_edge_idx);
        self.in_edges.push(in_edge);
    }
}
