use crate::graph::visualization::dot::{NodeShape, node_style::NodeStyle};
use alloc::string::String;

#[derive(Default)]
pub struct NodeSettings {
    pub shape: Option<NodeShape>,
    pub style: Option<NodeStyle>,
    pub fill_color: Option<String>,
}
