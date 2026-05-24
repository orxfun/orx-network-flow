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

    pub fn edges_idx(&self) -> usize {
        self.edges_idx
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn head_in_edge_idx(&self) -> usize {
        self.head_in_edge_idx
    }
}
