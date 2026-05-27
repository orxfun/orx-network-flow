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

        for v in self.vertices.indices() {
            let vertex = &self.vertices[v];
            let label = <(V, E) as DotData>::vertex_label(self, v, vertex);
            dot.push_str(&format!("    {v} [label=\"{label}\"];\n"));
        }

        for edge in self.edges.iter() {
            dot.push_str(&format!("    {} -> {};\n", edge.tail(), edge.head()));
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
        v.to_string()
    }
}
