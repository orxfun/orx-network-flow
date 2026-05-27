use crate::Graph;
use crate::graph::vertex::{VIdx, Vertex};
use alloc::{format, string::String};
use std::string::ToString;

impl<V, E> Graph<V, E>
where
    (V, E): DotData<V = V, E = E>,
{
    pub fn to_dot_string(&self) -> String {
        let mut dot = String::from("digraph G {\n");

        for vertex_idx in 0..self.vertices.len() {
            let v = VIdx(vertex_idx);
            let vertex = &self.vertices[vertex_idx];
            let label = <(V, E) as DotData>::vertex_label(self, v, vertex);
            dot.push_str(&format!("    {vertex_idx} [label=\"{label}\"];\n"));
        }

        for edge in &self.edges {
            dot.push_str(&format!("    {} -> {};\n", edge.tail().0, edge.head().0));
        }

        dot.push('}');
        dot
    }
}

pub trait DotData {
    type V;

    type E;

    fn vertex_label(graph: &Graph<Self::V, Self::E>, v: VIdx, vertex: &Vertex<Self::V>) -> String;
}

impl DotData for ((), ()) {
    type V = ();

    type E = ();

    fn vertex_label(_: &Graph<Self::V, Self::E>, v: VIdx, _: &Vertex<Self::V>) -> String {
        v.0.to_string()
    }
}
