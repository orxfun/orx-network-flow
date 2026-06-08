use crate::graphs::visualization::dot::DotGraph;

pub trait AsDotGraph {
    type Settings;

    fn as_dot_graph(&self) -> impl DotGraph;

    fn as_dot_graph_with_settings(&self, settings: Self::Settings) -> impl DotGraph;
}
