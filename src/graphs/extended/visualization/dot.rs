use crate::graphs::Graph;
use crate::graphs::extended::GraphExtended;
use crate::graphs::visualization::dot::{AsDotGraph, DotGraph, DotGraphBasic, NodeSettings};

impl<'a, G, Dv, De> AsDotGraph for GraphExtended<'a, G, Dv, De>
where
    G: Graph,
{
    type Settings = NodeSettings;

    fn as_dot_graph(&self) -> impl DotGraph {
        DotGraphBasic::new(self)
    }

    fn as_dot_graph_with_settings(&self, settings: Self::Settings) -> impl DotGraph {
        DotGraphBasic::new_with_settings(self, settings)
    }
}
