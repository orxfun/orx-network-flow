use crate::graphs::visualization::dot::edge_settings::EdgeSettings;
use crate::graphs::visualization::dot::{DotGraph, VertexSettings};
use crate::graphs::{EIdx, Graph, VIdx};
use alloc::string::ToString;
use core::fmt::Display;

pub struct DotGraphBasic<'a, G: Graph> {
    g: &'a G,
    vertex: VertexSettings,
    edge: EdgeSettings,
}

impl<'a, G: Graph> DotGraphBasic<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        DotGraphBasic::new_with_settings(graph, Default::default(), Default::default())
    }

    pub fn new_with_settings(g: &'a G, vertex: VertexSettings, edge: EdgeSettings) -> Self {
        DotGraphBasic { g, vertex, edge }
    }
}

impl<'a, G: Graph> DotGraph for DotGraphBasic<'a, G> {
    type G = G;

    fn vertex_label(&self, v: VIdx) -> impl Display {
        v.to_string()
    }

    fn vertex_settings(&self, _: VIdx) -> &VertexSettings {
        &self.vertex
    }

    fn edge_settings(&self, _: EIdx) -> &EdgeSettings {
        &self.edge
    }

    fn graph(&self) -> &G {
        self.g
    }
}
