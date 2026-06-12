use crate::graphs::visualization::dot::{
    DotGraph, EdgeSettings, VertexSettings, VertexShape, VertexStyle,
};
use crate::graphs::{Graph, VIdx, Vertex};
use crate::networks::com_by_od_st_nw::{nw::ComOdStNw, vertex_data::ComOdStDv};
use crate::networks::transport_nw::visualization::dot::dot_vertex_label;
use crate::space_time::SpaceTimeOd;
use crate::{Problem, Variant};
use alloc::{format, string::ToString};

pub struct DotComOdStNw<'a, V: Variant> {
    p: &'a Problem<V>,
    nw: &'a ComOdStNw<'a, V>,
    transport_settings: VertexSettings,
    source_settings: VertexSettings,
    sink_settings: VertexSettings,
}

impl<'a, V: Variant> DotComOdStNw<'a, V> {
    pub fn new(
        p: &'a Problem<V>,
        nw: &'a ComOdStNw<'a, V>,
        transport_settings: Option<VertexSettings>,
        source_settings: Option<VertexSettings>,
        sink_settings: Option<VertexSettings>,
    ) -> Self {
        let transport_settings = transport_settings.unwrap_or(VertexSettings {
            shape: Some(VertexShape::Rect),
            ..Default::default()
        });

        let source_settings = source_settings.unwrap_or(VertexSettings {
            shape: Some(VertexShape::House),
            style: Some(VertexStyle::Filled),
            fill_color: Some("lightgreen".to_string()),
            ..Default::default()
        });

        let sink_settings = sink_settings.unwrap_or(VertexSettings {
            shape: Some(VertexShape::InvHouse),
            style: Some(VertexStyle::Filled),
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
    type G = ComOdStNw<'a, V>;

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

    fn graph(&self) -> &Self::G {
        self.nw
    }

    fn vertex_settings(&self, v: VIdx) -> &VertexSettings {
        match self.nw.vertex(v).data() {
            ComOdStDv::Transport(_) => &self.transport_settings,
            ComOdStDv::OriSt(_, _) => &self.source_settings,
            ComOdStDv::DesSt(_, _) => &self.sink_settings,
        }
    }

    fn edge_settings(&self, e: crate::graphs::EIdx) -> &EdgeSettings {
        todo!()
    }

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.nw.vertex_indices()
    }
}
