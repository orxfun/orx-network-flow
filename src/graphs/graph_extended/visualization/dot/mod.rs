use crate::graphs::graph_core::visualization::dot::DotGraph;
use crate::graphs::graph_extended::ext_graph::ExtGraph;
use crate::graphs::{GraphCore, VIdx, Vertex};

impl<'a, V, E, Ve, Ee> DotGraph for ExtGraph<'a, V, E, Ve, Ee> {
    type V = Ve;

    type E = Ee;

    fn graph(&self) -> &GraphCore<Self::V, Self::E> {
        todo!()
    }

    fn vertex_label(&self, v: VIdx, vertex: &Vertex<Self::V>) -> std::prelude::v1::String {
        todo!()
    }

    fn vertex_settings(&self, v: VIdx, vertex: &Vertex<Self::V>) -> std::prelude::v1::String {
        todo!()
    }

    fn vertex_tooltip(
        &self,
        v: VIdx,
        vertex: &Vertex<Self::V>,
    ) -> Option<std::prelude::v1::String> {
        todo!()
    }
}
