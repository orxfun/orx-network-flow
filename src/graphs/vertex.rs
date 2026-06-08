use crate::graphs::edge::EIdx;
use crate::graphs::{in_edge::InEdge, out_edge::OutEdge};
use crate::{impl_idx, impl_vec_of_idx};
use alloc::vec::Vec;

impl_idx!(VIdx);
impl_vec_of_idx!(VIdx, VecVertex);

pub struct Vertex<V> {
    data: V,
    out_edges: Vec<OutEdge>,
    in_edges: Vec<InEdge>,
}

impl<V> Vertex<V> {
    pub fn new(data: V) -> Self {
        Self {
            data,
            out_edges: Vec::new(),
            in_edges: Vec::new(),
        }
    }

    pub fn add_out_edge(&mut self, edges_idx: EIdx, head: VIdx, head_in_edge_idx: usize) {
        let out_edge = OutEdge::new(edges_idx, head, head_in_edge_idx);
        self.out_edges.push(out_edge);
    }

    pub fn add_in_edge(&mut self, edges_idx: EIdx, tail: VIdx, tail_out_edge_idx: usize) {
        let in_edge = InEdge::new(edges_idx, tail, tail_out_edge_idx);
        self.in_edges.push(in_edge);
    }

    pub fn out_edges(&self) -> &[OutEdge] {
        &self.out_edges
    }

    pub fn in_edges(&self) -> &[InEdge] {
        &self.in_edges
    }

    pub fn data(&self) -> &V {
        &self.data
    }
}
