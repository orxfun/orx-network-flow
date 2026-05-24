use crate::graph::{in_edge::InEdge, out_edge::OutEdge};
use alloc::vec::Vec;

pub struct Node<N> {
    data: N,
    out_edges: Vec<OutEdge>,
    in_edges: Vec<InEdge>,
}
