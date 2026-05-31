use crate::graph::visualization::dot::{NodeShape, node_style::NodeStyle};
use alloc::string::String;

pub struct NodeSettings {
    shape: Option<NodeShape>,
    style: Option<NodeStyle>,
    fill_color: String,
}
