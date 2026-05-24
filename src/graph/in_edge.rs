pub struct InEdge {
    edges_idx: usize,
    tail: usize,
    tail_out_edge_idx: usize,
}

impl InEdge {
    pub fn new(edges_idx: usize, tail: usize, tail_out_edge_idx: usize) -> Self {
        Self {
            edges_idx,
            tail,
            tail_out_edge_idx,
        }
    }
}
