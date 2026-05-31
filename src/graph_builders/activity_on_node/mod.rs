mod builder;
mod dot_graph;
mod dot_settings;
mod edge;
mod indexer;
mod vertex;

pub use builder::build_aon_graph;
pub use dot_graph::AonDotGraph;
pub use edge::EdgeData;
pub use vertex::VertexData;

pub type AonGraph = crate::Graph<VertexData, EdgeData>;
