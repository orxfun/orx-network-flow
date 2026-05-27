use crate::graph::{builder::GraphBuilder, edge::Edge, vertex::Vertex};
use alloc::{format, string::String, vec::Vec};

pub struct Graph<V, E> {
    pub(super) nodes: Vec<Vertex<V>>,
    pub(super) edges: Vec<Edge<E>>,
}

impl<V, E> Graph<V, E> {
    pub fn builder(num_nodes: usize, data: impl Fn(usize) -> V) -> GraphBuilder<V, E> {
        GraphBuilder::new(num_nodes, data)
    }

    pub fn to_dot_string(&self) -> String {
        let mut dot = String::from("digraph G {\n");

        for node_idx in 0..self.nodes.len() {
            dot.push_str(&format!("    {node_idx};\n"));
        }

        for edge in &self.edges {
            dot.push_str(&format!("    {} -> {};\n", edge.tail(), edge.head()));
        }

        dot.push('}');
        dot
    }
}
