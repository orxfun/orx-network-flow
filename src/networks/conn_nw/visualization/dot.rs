use crate::Variant;
use crate::graphs::visualization::dot::{DotGraph, NodeSettings, NodeShape};
use crate::graphs::{Graph, VIdx};
use crate::networks::conn_nw::nw::ConnNw;
use alloc::format;

const TRANSPORT_SETTINGS: NodeSettings = NodeSettings {
    shape: Some(NodeShape::Rect),
    style: None,
    fill_color: None,
};

pub struct DotConnNw<'a, V: Variant> {
    nw: &'a ConnNw<'a, V>,
    transport_settings: NodeSettings,
}

impl<'a, V> DotConnNw<'a, V>
where
    V: Variant,
{
    pub fn new(nw: &'a ConnNw<'a, V>, transport_settings: Option<NodeSettings>) -> Self {
        Self {
            nw,
            transport_settings: transport_settings.unwrap_or(TRANSPORT_SETTINGS),
        }
    }
}

impl<'a, V> DotGraph for DotConnNw<'a, V>
where
    V: Variant,
{
    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        format!("{v}")
    }

    fn vertex_settings(&self, v: VIdx) -> &NodeSettings {
        &self.transport_settings
    }

    fn graph(&self) -> &impl Graph {
        &self.nw.g
    }
}
