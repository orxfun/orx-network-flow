use crate::graph::visualization::dot::{NodeSettings, NodeShape, NodeStyle};
use alloc::string::ToString;

pub struct AonDotGraphSettings {
    pub source: NodeSettings,
    pub sink: NodeSettings,
    pub transport: NodeSettings,
}

impl Default for AonDotGraphSettings {
    fn default() -> Self {
        Self {
            source: NodeSettings {
                shape: Some(NodeShape::House),
                style: Some(NodeStyle::Filled),
                fill_color: Some("chartreuse".to_string()),
            },
            sink: NodeSettings {
                shape: Some(NodeShape::InvHouse),
                style: Some(NodeStyle::Filled),
                fill_color: Some("tomato".to_string()),
            },
            transport: NodeSettings {
                shape: Some(NodeShape::Rect),
                ..Default::default()
            },
        }
    }
}
