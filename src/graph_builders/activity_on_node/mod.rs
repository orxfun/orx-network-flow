#[cfg(test)]
mod tests;

mod builder;
mod edge;
mod indexer;
mod vertex;

pub use builder::build_aon_graph;
pub use edge::EdgeData;
pub use vertex::VertexData;

pub type AonGraph = crate::Graph<VertexData, EdgeData>;
