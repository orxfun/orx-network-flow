use crate::graph::visualization::dot::{NodeSettings, NodeShape, NodeStyle};
use alloc::string::ToString;

pub struct AonDotGraphSettings {
    source: NodeSettings,
    sink: NodeSettings,
    transport: NodeSettings,
}

impl Default for AonDotGraphSettings {
    fn default() -> Self {
        Self {
            source: NodeSettings {
                shape: Some(NodeShape::Doublecircle),
                style: Some(NodeStyle::Filled),
                fill_color: Some("lightblue".to_string()),
            },
            sink: NodeSettings {
                shape: Some(NodeShape::Doublecircle),
                style: Some(NodeStyle::Filled),
                fill_color: Some("olive".to_string()),
            },
            transport: NodeSettings {
                shape: Some(NodeShape::Rect),
                ..Default::default()
            },
        }
    }
}
