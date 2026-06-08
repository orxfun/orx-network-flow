use crate::graphs::visualization::dot::{DotGraph, NodeSettings};
use crate::graphs::{Edge, Graph, VIdx};
use alloc::string::ToString;
use core::fmt::Display;

pub struct DotGraphBasic<'a, G: Graph> {
    graph: &'a G,
    settings: NodeSettings,
}

impl<'a, G: Graph> DotGraphBasic<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        DotGraphBasic::new_with_settings(graph, Default::default())
    }

    pub fn new_with_settings(graph: &'a G, settings: NodeSettings) -> Self {
        DotGraphBasic { graph, settings }
    }
}

impl<'a, G: Graph> DotGraph for DotGraphBasic<'a, G> {
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
        self.graph.edges().map(|e| (e.tail(), e.head()))
    }
}
