use crate::Variant;
use crate::graphs::visualization::dot::{DotGraph, NodeSettings, NodeShape, NodeStyle};
use crate::graphs::{Graph, VIdx, Vertex};
use crate::networks::conn_wait_nw::{ConnWaitGraph, ConnWaitNw};
use alloc::{format, string::String};

fn default_transport() -> NodeSettings {
    NodeSettings {
        shape: Some(NodeShape::Rect),
        style: Some(NodeStyle::Filled),
        fill_color: Some(String::from("lightgreen")),
    }
}

pub struct ConnWaitDot<'a, V>
where
    V: Variant,
{
    nw: &'a ConnWaitNw<'a, V>,
    transport: NodeSettings,
}

impl<'a, V> ConnWaitDot<'a, V>
where
    V: Variant,
{
    pub fn new(nw: &'a ConnWaitNw<'a, V>, transport: Option<NodeSettings>) -> Self {
        Self {
            nw,
            transport: transport.unwrap_or_else(default_transport),
        }
    }
}

impl<'a, V> DotGraph for ConnWaitDot<'a, V>
where
    V: Variant,
{
    type G = ConnWaitGraph;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        match self.graph().vertex(v).data() {
            _ => format!("{v}"),
        }
    }

    fn vertex_settings(&self, v: VIdx) -> &NodeSettings {
        match self.graph().vertex(v).data() {
            _ => todo!("vertex settings"),
        }
    }

    fn graph(&self) -> &Self::G {
        self.nw.g()
    }
}
