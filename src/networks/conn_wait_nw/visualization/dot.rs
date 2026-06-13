use crate::Variant;
use crate::graphs::visualization::dot::{
    DotGraph, EdgeSettings, VertexSettings, VertexShape, VertexStyle,
};
use crate::graphs::{EIdx, Edge, Graph, VIdx, Vertex};
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitGraph, ConnWaitNw, ConnWaitVertex};
use crate::spaces::Space;
use alloc::{format, string::String};

pub struct ConnWaitDotSettings {
    transport: VertexSettings,
    ready_ori: VertexSettings,
    due_des: VertexSettings,
    wait: EdgeSettings,
    connect: EdgeSettings,
    enter: EdgeSettings,
    exit: EdgeSettings,
    bypass: EdgeSettings,
}

impl Default for ConnWaitDotSettings {
    fn default() -> Self {
        Self {
            transport: VertexSettings {
                shape: Some(VertexShape::Rect),
                style: None,
                fill_color: None,
            },
            ready_ori: VertexSettings {
                shape: Some(VertexShape::Circle),
                style: Some(VertexStyle::Filled),
                fill_color: Some(String::from("lightgreen")),
            },
            due_des: VertexSettings {
                shape: Some(VertexShape::Circle),
                style: Some(VertexStyle::Filled),
                fill_color: Some(String::from("tomato")),
            },
            wait: EdgeSettings {
                color: Some(String::from("lightgray")),
            },
            connect: EdgeSettings {
                color: Some(String::from("darkgreen")),
            },
            enter: EdgeSettings {
                color: Some(String::from("lightgray")),
            },
            exit: EdgeSettings {
                color: Some(String::from("darkgreen")),
            },
            bypass: EdgeSettings {
                color: Some(String::from("orange")),
            },
        }
    }
}

pub struct ConnWaitDot<'a, V>
where
    V: Variant,
{
    nw: &'a ConnWaitNw<'a, V>,
    settings: ConnWaitDotSettings,
}

impl<'a, V> ConnWaitDot<'a, V>
where
    V: Variant,
{
    pub fn new(nw: &'a ConnWaitNw<'a, V>, settings: Option<ConnWaitDotSettings>) -> Self {
        Self {
            nw,
            settings: settings.unwrap_or_default(),
        }
    }

    fn space(&self, space: Space) -> &V::S {
        self.nw.p.space_key(space)
    }
}

impl<'a, V> DotGraph for ConnWaitDot<'a, V>
where
    V: Variant,
{
    type G = ConnWaitGraph;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        let p = self.nw.p;
        match self.graph().vertex(v).data() {
            ConnWaitVertex::Transport(t) => {
                let data = p.transport_by_idx(*t);
                format!(
                    "{}\n{}-{}\n{}-{}",
                    t,
                    self.space(data.origin().space()),
                    self.space(data.destination().space()),
                    data.origin().time(),
                    data.destination().time()
                )
            }
            ConnWaitVertex::ReadyOri(ro, _) => {
                let ori = p.space_key(ro.space());
                format!("{}\n{}-{}", v, ori, ro.time())
            }
            ConnWaitVertex::DueDes(ro, _) => {
                let des = p.space_key(ro.space());
                format!("{}\n{}-{}", v, des, ro.time())
            }
            _ => format!("{v}"),
        }
    }

    fn vertex_settings(&self, v: VIdx) -> &VertexSettings {
        match self.graph().vertex(v).data() {
            ConnWaitVertex::Transport(_) => &self.settings.transport,
            ConnWaitVertex::ReadyOri(_, _) => &self.settings.ready_ori,
            ConnWaitVertex::DueDes(_, _) => &self.settings.due_des,
            _ => todo!("vertex settings"),
        }
    }

    fn edge_label(&self, _: EIdx) -> impl core::fmt::Display {
        String::new()
    }

    fn edge_settings(&self, e: EIdx) -> &EdgeSettings {
        match self.graph().edge(e).data() {
            ConnWaitEdge::Wait => &self.settings.wait,
            ConnWaitEdge::Connect => &self.settings.connect,
            ConnWaitEdge::Enter => &self.settings.enter,
            ConnWaitEdge::Exit => &self.settings.exit,
            ConnWaitEdge::Bypass(_) => &self.settings.bypass,
        }
    }

    fn graph(&self) -> &Self::G {
        &self.nw.g
    }
}
