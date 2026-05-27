use crate::graph::{builder::GraphBuilder, edge::Edge, vertex::Vertex};
use alloc::{format, string::String, vec::Vec};

pub struct Graph<V, E> {
    pub(super) vertices: Vec<Vertex<V>>,
    pub(super) edges: Vec<Edge<E>>,
}

impl<V, E> Graph<V, E> {
    pub fn builder(vertices: impl Iterator<Item = V>) -> GraphBuilder<V, E> {
        GraphBuilder::new(vertices)
    }

    pub fn to_dot_string(&self) -> String {
        let mut dot = String::from("digraph G {\n");

        for vertex_idx in 0..self.vertices.len() {
            dot.push_str(&format!("    {vertex_idx};\n"));
        }

        for edge in &self.edges {
            dot.push_str(&format!("    {} -> {};\n", edge.tail().0, edge.head().0));
        }

        dot.push('}');
        dot
    }
}
