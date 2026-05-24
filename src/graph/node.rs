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
}
