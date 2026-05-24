use crate::graph::{edge::Edge, graph::Graph, node::Node};
use alloc::vec::Vec;

pub struct GraphBuilder<N, E>(Graph<N, E>);

impl<N, E> GraphBuilder<N, E> {
    pub fn new(num_nodes: usize, data: impl Fn(usize) -> N) -> Self {
        let nodes: Vec<_> = (0..num_nodes).map(data).map(Node::new).collect();
        let edges = Vec::new();
        let graph = Graph { nodes, edges };
        Self(graph)
    }

    pub fn edge(&mut self, data: E, tail: usize, head: usize) {
        let edges_idx = self.0.edges.len();
        let tail_out_edge_idx = self.0.nodes[tail].out_edges().len();
        let head_in_edge_idx = self.0.nodes[head].in_edges().len();
        self.0.edges.push(Edge::new(tail, head, data));
        self.0.nodes[tail].add_out_edge(edges_idx, head, head_in_edge_idx);
        self.0.nodes[head].add_in_edge(edges_idx, tail, tail_out_edge_idx);
    }
}
