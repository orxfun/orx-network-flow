use crate::Graph;
use crate::graph::{VIdx, Vertex};
use alloc::format;
use alloc::string::{String, ToString};

pub trait DotGraph {
    type V;

    type E;

    fn graph(&self) -> &Graph<Self::V, Self::E>;

    fn vertex_label(
        &self,
        graph: &Graph<Self::V, Self::E>,
        v: VIdx,
        vertex: &Vertex<Self::V>,
    ) -> String;

    fn to_dot_string(&self) -> String {
        let gr = self.graph();

        let mut dot = String::from("digraph G {\n");

        for v in gr.vertices.indices() {
            let vertex = &gr.vertices[v];
            let label = self.vertex_label(gr, v, vertex);
            dot.push_str(&format!("    {v} [label=\"{label}\"];\n"));
        }

        for edge in gr.edges.iter() {
            dot.push_str(&format!("    {} -> {};\n", edge.tail(), edge.head()));
        }

        dot.push('}');
        dot
    }
}

impl DotGraph for Graph<(), ()> {
    type V = ();

    type E = ();

    fn graph(&self) -> &Graph<Self::V, Self::E> {
        self
    }

    fn vertex_label(&self, _: &Graph<Self::V, Self::E>, v: VIdx, _: &Vertex<Self::V>) -> String {
        v.to_string()
    }
}
