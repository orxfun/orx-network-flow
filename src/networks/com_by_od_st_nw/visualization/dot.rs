use crate::graphs::visualization::dot::{DotGraph, NodeSettings, NodeShape, NodeStyle};
use crate::graphs::{Edge, Graph, VIdx, Vertex};
use crate::networks::com_by_od_st_nw::{nw::ComOdStNw, vertex_data::ComOdStDv};
use crate::networks::transport_nw::visualization::dot::dot_vertex_label;
use crate::space_time::SpaceTimeOd;
use crate::{Problem, Variant};
use alloc::{format, string::ToString};

pub struct DotComOdStNw<'a, V: Variant> {
    p: &'a Problem<V>,
    nw: &'a ComOdStNw<'a, V>,
    transport_settings: NodeSettings,
    source_settings: NodeSettings,
    sink_settings: NodeSettings,
}

impl<'a, V: Variant> DotComOdStNw<'a, V> {
    pub fn new(
        p: &'a Problem<V>,
        nw: &'a ComOdStNw<'a, V>,
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
            style: Some(NodeStyle::Filled),
            fill_color: Some("lightgreen".to_string()),
            ..Default::default()
        });

        let sink_settings = sink_settings.unwrap_or(NodeSettings {
            shape: Some(NodeShape::InvHouse),
            style: Some(NodeStyle::Filled),
            fill_color: Some("tomato".to_string()),
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

        let end = |st: &SpaceTimeOd, v| {
            format!(
                "{}\n{}-{}\n{}-{}",
                v,
                p.space_key(st.ori.space()),
                p.space_key(st.des.space()),
                st.ori.time(),
                st.des.time()
            )
        };

        match vertex.data() {
            ComOdStDv::Transport(t) => dot_vertex_label(p, v, *t),
            ComOdStDv::OriSt(st, _) => end(st, v),
            ComOdStDv::DesSt(st, _) => end(st, v),
        }
    }

    fn vertex_tooltip(&self, v: VIdx) -> Option<impl core::fmt::Display> {
        let p = self.p;
        let vertex = self.nw.vertex(v);

        let end = |amount: &V::F| format!("total amount = {}", amount);

        Some(match vertex.data() {
            ComOdStDv::Transport(t) => dot_vertex_label(p, v, *t),
            ComOdStDv::OriSt(_, amount) => end(amount),
            ComOdStDv::DesSt(_, amount) => end(amount),
        })
    }

    fn vertex_settings(&self, v: VIdx) -> &NodeSettings {
        match self.nw.vertex(v).data() {
            ComOdStDv::Transport(_) => &self.transport_settings,
            ComOdStDv::OriSt(_, _) => &self.source_settings,
            ComOdStDv::DesSt(_, _) => &self.sink_settings,
        }
    }

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.nw.vertex_indices()
    }

    fn edges(&self) -> impl Iterator<Item = (VIdx, VIdx)> {
        self.nw.edges().map(|x| (x.tail(), x.head()))
    }
}
