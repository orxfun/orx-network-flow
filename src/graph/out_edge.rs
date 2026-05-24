pub struct OutEdge {
    edges_idx: usize,
    head: usize,
    head_in_edge_idx: usize,
}

impl OutEdge {
    pub fn new(edges_idx: usize, head: usize, head_in_edge_idx: usize) -> Self {
        Self {
            edges_idx,
            head,
            head_in_edge_idx,
        }
    }
}
