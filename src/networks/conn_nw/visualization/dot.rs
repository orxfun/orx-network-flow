use crate::Variant;
use crate::graphs::visualization::dot::{DotGraph, NodeSettings, NodeShape, NodeStyle};
use crate::graphs::{Graph, VIdx, Vertex};
use crate::networks::conn_nw::nw::{ConnNw, ConnNwGr};
use crate::networks::conn_nw::vertex_data::ConnNwVertex;
use alloc::{format, string::String};

fn default_dt_ori_settings() -> NodeSettings {
    NodeSettings {
        shape: Some(NodeShape::Rect),
        style: Some(NodeStyle::Filled),
        fill_color: Some(String::from("lightgreen")),
    }
}

fn default_at_des_settings() -> NodeSettings {
    NodeSettings {
        shape: Some(NodeShape::Rect),
        style: Some(NodeStyle::Filled),
        fill_color: Some(String::from("tomato")),
    }
}

pub struct DotConnNw<'a, V: Variant> {
    nw: &'a ConnNw<'a, V>,
    dt_ori_settings: NodeSettings,
    at_des_settings: NodeSettings,
}

impl<'a, V> DotConnNw<'a, V>
where
    V: Variant,
{
    pub fn new(
        nw: &'a ConnNw<'a, V>,
        dt_ori_settings: Option<NodeSettings>,
        at_des_settings: Option<NodeSettings>,
    ) -> Self {
        Self {
            nw,
            dt_ori_settings: dt_ori_settings.unwrap_or_else(default_dt_ori_settings),
            at_des_settings: at_des_settings.unwrap_or_else(default_at_des_settings),
        }
    }
}

impl<'a, V> DotGraph for DotConnNw<'a, V>
where
    V: Variant,
{
    type G = ConnNwGr;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        match self.graph().vertex(v).data() {
            ConnNwVertex::St(dt_ori) => {
                let ori = self.nw.p().space_key(dt_ori.space());
                format!("{} - {}", ori, dt_ori.time())
            }
            ConnNwVertex::AtDes(at_des) => {
                let des = self.nw.p().space_key(at_des.space());
                format!("{} - {}", des, at_des.time())
            }
            _ => format!("{v}"),
        }
    }

    fn vertex_settings(&self, v: VIdx) -> &NodeSettings {
        match self.graph().vertex(v).data() {
            ConnNwVertex::St(_) => &self.dt_ori_settings,
            ConnNwVertex::AtDes(_) => &self.at_des_settings,
            _ => todo!("vertex settings"),
        }
    }

    fn graph(&self) -> &Self::G {
        &self.nw.g
    }
}
