use crate::graph::{DotGraph, VIdx, Vertex};
use crate::graph_builders::activity_on_node::{EdgeData, VertexData};
use crate::{Graph, Problem, Variant};
use alloc::format;
use alloc::string::String;

// impl DotData for (VertexData, EdgeData) {
//     type V = VertexData;

//     type E = EdgeData;

//     fn vertex_label(_: &Graph<Self::V, Self::E>, v: VIdx, vertex: &Vertex<Self::V>) -> String {
//         match vertex.data() {
//             VertexData::Transport(t) => {
//                 // TODO: abc

//                 format!("{}\nm{}", v, t)
//             }
//             VertexData::Source(c) => format!("{}\ns{}", v, c),
//             VertexData::Sink(c) => format!("{}\nt{}", v, c),
//         }
//     }
// }

pub struct AonDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    graph: &'a Graph<VertexData, EdgeData>,
}

impl<'a, V: Variant> AonDotGraph<'a, V> {
    pub fn new(problem: &'a Problem<V>, graph: &'a Graph<VertexData, EdgeData>) -> Self {
        Self { problem, graph }
    }
}

impl<'a, V: Variant> DotGraph for AonDotGraph<'a, V> {
    type V = VertexData;

    type E = EdgeData;

    fn graph(&self) -> &Graph<Self::V, Self::E> {
        self.graph
    }

    fn vertex_label(
        &self,
        graph: &Graph<Self::V, Self::E>,
        v: VIdx,
        vertex: &Vertex<Self::V>,
    ) -> String {
        // self.graph.todo
        td
    }
}
