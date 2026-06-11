use crate::Variant;
use crate::graphs::visualization::dot::{
    DotGraph, EdgeSettings, VertexSettings, VertexShape, VertexStyle,
};
use crate::graphs::{EIdx, Edge, Graph, VIdx, Vertex};
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitGraph, ConnWaitNw, ConnWaitVertex};
use crate::spaces::Space;
use alloc::{format, string::String};

fn default_transport() -> VertexSettings {
    VertexSettings {
        shape: Some(VertexShape::Rect),
        style: Some(VertexStyle::Filled),
        fill_color: Some(String::from("lightgreen")),
    }
}

fn default_wait() -> EdgeSettings {
    EdgeSettings {
        color: Some(String::from("lightgray")),
    }
}

fn default_connect() -> EdgeSettings {
    EdgeSettings {
        color: Some(String::from("green")),
    }
}

pub struct ConnWaitDot<'a, V>
where
    V: Variant,
{
    nw: &'a ConnWaitNw<'a, V>,
    transport: VertexSettings,
    wait: EdgeSettings,
    connect: EdgeSettings,
}

impl<'a, V> ConnWaitDot<'a, V>
where
    V: Variant,
{
    pub fn new(
        nw: &'a ConnWaitNw<'a, V>,
        transport: Option<VertexSettings>,
        wait: Option<EdgeSettings>,
        connect: Option<EdgeSettings>,
    ) -> Self {
        Self {
            nw,
            transport: transport.unwrap_or_else(default_transport),
            wait: wait.unwrap_or_else(default_wait),
            connect: connect.unwrap_or_else(default_connect),
        }
    }

    fn space(&self, space: Space) -> &V::S {
        self.nw.p().space_key(space)
    }
}

impl<'a, V> DotGraph for ConnWaitDot<'a, V>
where
    V: Variant,
{
    type G = ConnWaitGraph;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        match self.graph().vertex(v).data() {
            ConnWaitVertex::Transport(t) => {
                let data = self.nw.p().transport_by_idx(*t);
                format!(
                    "{}\n{}-{}\n{}-{}",
                    t,
                    self.space(data.origin().space()),
                    self.space(data.destination().space()),
                    data.origin().time(),
                    data.destination().time()
                )
            }
            _ => format!("{v}"),
        }
    }

    fn vertex_settings(&self, v: VIdx) -> &VertexSettings {
        match self.graph().vertex(v).data() {
            ConnWaitVertex::Transport(_) => &self.transport,
            _ => todo!("vertex settings"),
        }
    }

    fn edge_settings(&self, e: EIdx) -> &EdgeSettings {
        match self.graph().edge(e).data() {
            ConnWaitEdge::Wait => &self.wait,
            ConnWaitEdge::Connect => &self.connect,
        }
    }

    fn graph(&self) -> &Self::G {
        self.nw.g()
    }
}
