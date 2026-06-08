use crate::graphs::visualization::dot::{AsDotGraph, DotGraph, NodeSettings};
use crate::graphs::{Edge, Graph, VIdx, core::GraphCore};
use alloc::string::ToString;
use core::fmt::Display;

#[derive(derive_new::new)]
pub struct DotGraphCore<'a, Dv, De> {
    graph: &'a GraphCore<Dv, De>,
    settings: NodeSettings,
}

impl<Dv, De> DotGraph for DotGraphCore<'_, Dv, De> {
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

impl<V, E> AsDotGraph for GraphCore<V, E> {
    type Settings = NodeSettings;

    fn as_dot_graph(&self) -> impl DotGraph {
        DotGraphCore::new(self, Default::default())
    }

    fn as_dot_graph_with_settings(&self, settings: Self::Settings) -> impl DotGraph {
        DotGraphCore::new(self, settings)
    }
}
