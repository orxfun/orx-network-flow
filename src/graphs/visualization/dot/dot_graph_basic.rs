use crate::graphs::visualization::dot::{DotGraph, NodeSettings};
use crate::graphs::{Graph, VIdx};
use alloc::string::ToString;
use core::fmt::Display;

pub struct DotGraphBasic<'a, G: Graph> {
    g: &'a G,
    settings: NodeSettings,
}

impl<'a, G: Graph> DotGraphBasic<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        DotGraphBasic::new_with_settings(graph, Default::default())
    }

    pub fn new_with_settings(g: &'a G, settings: NodeSettings) -> Self {
        DotGraphBasic { g, settings }
    }
}

impl<'a, G: Graph> DotGraph for DotGraphBasic<'a, G> {
    type G = G;

    fn vertex_label(&self, v: VIdx) -> impl Display {
        v.to_string()
    }

    fn vertex_settings(&self, _: VIdx) -> &NodeSettings {
        &self.settings
    }

    fn graph(&self) -> &G {
        self.g
    }
}
