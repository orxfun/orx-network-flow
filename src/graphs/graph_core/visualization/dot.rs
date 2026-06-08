use crate::graphs::visualization::dot::{DotGraph, NodeSettings};
use crate::graphs::{Edge, Graph, GraphCore, VIdx};
use alloc::string::ToString;
use core::fmt::Display;

#[derive(derive_new::new)]
pub struct DotGraphCore<'a, V, E> {
    graph: &'a GraphCore<V, E>,
    settings: NodeSettings,
}

impl<V, E> DotGraph for DotGraphCore<'_, V, E> {
    fn vertex_label(&self, v: VIdx) -> impl Display {
        v.to_string()
    }

    fn vertex_settings(&self, _: VIdx) -> &NodeSettings {
        &self.settings
    }

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.graph.vertex_indices()
    }

    fn edges(&self) -> impl Iterator<Item = (VIdx, VIdx)> {
        self.graph.edges.iter().map(|e| (e.tail(), e.head()))
    }
}
