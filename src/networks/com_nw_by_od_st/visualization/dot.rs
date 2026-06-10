use crate::graphs::visualization::dot::{DotGraph, NodeSettings, NodeShape};
use crate::graphs::{Edge, Graph, VIdx, Vertex};
use crate::networks::com_nw_by_od_st::{nw::ComOdStNw, vertex_data::ComOdStDv};
use crate::networks::transport_nw::visualization::dot::dot_vertex_label;
use crate::{Problem, Variant};
use alloc::{format, string::ToString};

pub struct DotComOdStNw<'a, V: Variant> {
    p: &'a Problem<V>,
    nw: &'a ComOdStNw,
    transport_settings: NodeSettings,
    source_settings: NodeSettings,
    sink_settings: NodeSettings,
}

impl<'a, V: Variant> DotComOdStNw<'a, V> {
    pub fn new(
        p: &'a Problem<V>,
        nw: &'a ComOdStNw,
        transport_settings: Option<NodeSettings>,
        source_settings: Option<NodeSettings>,
        sink_settings: Option<NodeSettings>,
    ) -> Self {
        let transport_settings = transport_settings.unwrap_or(NodeSettings {
            shape: Some(NodeShape::Rect),
            ..Default::default()
        });

        let source_settings = source_settings.unwrap_or(NodeSettings {
            shape: Some(NodeShape::House),
            fill_color: Some("green".to_string()),
            ..Default::default()
        });

        let sink_settings = sink_settings.unwrap_or(NodeSettings {
            shape: Some(NodeShape::InvHouse),
            fill_color: Some("red".to_string()),
            ..Default::default()
        });

        Self {
            p,
            nw,
            transport_settings,
            source_settings,
            sink_settings,
        }
    }
}

impl<'a, V: Variant> DotGraph for DotComOdStNw<'a, V> {
    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        let p = self.p;
        let vertex = self.nw.vertex(v);
        match vertex.data() {
            ComOdStDv::Transport(t) => dot_vertex_label(p, v, *t),
            ComOdStDv::OriSt(st) => {
                format!("{}\n{}-{}", v, p.space_key(st.space()), st.time())
            }
            ComOdStDv::DesSt(st) => {
                format!("{}\n{}-{}", v, p.space_key(st.space()), st.time())
            }
        }
    }

    fn vertex_settings(&self, v: VIdx) -> &NodeSettings {
        match self.nw.vertex(v).data() {
            ComOdStDv::Transport(_) => &self.transport_settings,
            ComOdStDv::OriSt(_) => &self.source_settings,
            ComOdStDv::DesSt(_) => &self.sink_settings,
        }
    }

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.nw.vertex_indices()
    }

    fn edges(&self) -> impl Iterator<Item = (VIdx, VIdx)> {
        self.nw.edges().map(|x| (x.tail(), x.head()))
    }
}
