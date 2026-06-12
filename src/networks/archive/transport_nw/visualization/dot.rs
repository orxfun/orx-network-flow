use crate::graphs::visualization::dot::{DotGraph, VertexSettings, VertexShape};
use crate::graphs::{Graph, VIdx, Vertex};
use crate::networks::TrNw;
use crate::transports::Transport;
use crate::{Problem, Variant};
use alloc::{format, string::String};

pub struct DotTrNw<'a, V: Variant> {
    p: &'a Problem<V>,
    nw: &'a TrNw<V>,
    node_settings: VertexSettings,
}

impl<'a, V: Variant> DotTrNw<'a, V> {
    pub fn new(p: &'a Problem<V>, nw: &'a TrNw<V>, node_settings: Option<VertexSettings>) -> Self {
        let node_settings = node_settings.unwrap_or(VertexSettings {
            shape: Some(VertexShape::Rect),
            ..Default::default()
        });
        Self {
            p,
            nw,
            node_settings,
        }
    }
}

impl<'a, V: Variant> DotGraph for DotTrNw<'a, V> {
    type G = TrNw<V>;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        let t = self.nw.vertex(v).data().t;
        dot_vertex_label(self.p, v, t)
    }

    fn vertex_settings(&self, _: VIdx) -> &VertexSettings {
        &self.node_settings
    }

    fn edge_settings(
        &self,
        e: crate::graphs::EIdx,
    ) -> &crate::graphs::visualization::dot::EdgeSettings {
        todo!()
    }

    fn graph(&self) -> &Self::G {
        self.nw
    }

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.nw.vertex_indices()
    }
}

pub(crate) fn dot_vertex_label<V: Variant>(p: &Problem<V>, v: VIdx, t: Transport) -> String {
    let transport = p.transport_by_idx(t);
    let ori = p.space_key(transport.origin().space());
    let des = p.space_key(transport.destination().space());
    let dt = transport.origin().time();
    let at = transport.destination().time();
    format!("{}\n{}-{}\n{}-{}", v, ori, des, dt, at)
}
