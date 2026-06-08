use crate::graphs::{EIdx, Vertex};
use alloc::vec::Vec;

pub struct VertexCore<Dv> {
    out_edges: Vec<EIdx>,
    in_edges: Vec<EIdx>,
    data: Dv,
}

impl<V> VertexCore<V> {
    pub fn new(data: V) -> Self {
        Self {
            data,
            out_edges: Vec::new(),
            in_edges: Vec::new(),
        }
    }

    pub fn add_out_edge(&mut self, edges_idx: EIdx) {
        self.out_edges.push(edges_idx);
    }

    pub fn add_in_edge(&mut self, edges_idx: EIdx) {
        self.in_edges.push(edges_idx);
    }
}

impl<V> Vertex for VertexCore<V> {
    type Data = V;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn out_edges(&self) -> impl Iterator<Item = EIdx> {
        self.out_edges.iter().copied()
    }

    fn in_edges(&self) -> impl Iterator<Item = EIdx> {
        self.in_edges.iter().copied()
    }

    fn len_out_edges(&self) -> usize {
        self.out_edges.len()
    }

    fn len_in_edges(&self) -> usize {
        self.in_edges.len()
    }
}
