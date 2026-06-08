use crate::{Graph, VIdx};
use crate::{graph::Vertex, graph_extended::ext_graph::ExtGraph, visualization::dot::DotGraph};

impl<'a, V, E, Ve, Ee> DotGraph for ExtGraph<'a, V, E, Ve, Ee> {
    type V = Ve;

    type E = Ee;

    fn graph(&self) -> &Graph<Self::V, Self::E> {
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
