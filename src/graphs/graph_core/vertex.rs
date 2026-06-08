use crate::graphs::graph_core::{in_edge::InEdge, out_edge::OutEdge};
use crate::graphs::{EIdx, VIdx, Vertex};
use alloc::vec::Vec;

pub struct VertexCore<V> {
    out_edges: Vec<OutEdge>,
    in_edges: Vec<InEdge>,
    data: V,
}

impl<V> VertexCore<V> {
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
}

impl<V> Vertex for &VertexCore<V> {
    type Data = V;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn out_edges(&self) -> impl ExactSizeIterator<Item = EIdx> {
        self.out_edges.iter().map(|x| x.edges_idx())
    }

    fn in_edges(&self) -> impl ExactSizeIterator<Item = EIdx> {
        self.in_edges.iter().map(|x| x.edges_idx())
    }
}
