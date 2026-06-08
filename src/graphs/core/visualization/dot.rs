use crate::graphs::core::GraphCore;
use crate::graphs::visualization::dot::{AsDotGraph, DotGraph, DotGraphBasic, NodeSettings};

impl<V, E> AsDotGraph for GraphCore<V, E> {
    type Settings = NodeSettings;

    fn as_dot_graph(&self) -> impl DotGraph {
        DotGraphBasic::new(self)
    }

    fn as_dot_graph_with_settings(&self, settings: Self::Settings) -> impl DotGraph {
        DotGraphBasic::new_with_settings(self, settings)
    }
}
