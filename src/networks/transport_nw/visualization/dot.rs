use crate::graphs::visualization::dot::{DotGraph, NodeSettings, NodeShape};
use crate::graphs::{Edge, Graph, VIdx, Vertex};
use crate::networks::TrNw;
use crate::{Problem, Variant};
use alloc::format;

// dot graph

pub struct DotTrNw<'a, V: Variant> {
    p: &'a Problem<V>,
    nw: &'a TrNw<V>,
    node_settings: NodeSettings,
}

impl<'a, V: Variant> DotTrNw<'a, V> {
    pub fn new(p: &'a Problem<V>, nw: &'a TrNw<V>, node_settings: Option<NodeSettings>) -> Self {
        let node_settings = node_settings.unwrap_or(NodeSettings {
            shape: Some(NodeShape::Rect),
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
    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        let p = self.p;
        let vertex = self.nw.vertex(v);
        let t = vertex.data().t;
        let transport = p.transport_by_idx(t);
        let ori = p.space_key(transport.origin().space());
        let des = p.space_key(transport.destination().space());
        let dt = transport.origin().time();
        let at = transport.destination().time();
        format!("{}\n{}-{}\n{}-{}", v, ori, des, dt, at)
    }

    fn vertex_settings(&self, _: VIdx) -> &NodeSettings {
        &self.node_settings
    }

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.nw.vertex_indices()
    }

    fn edges(&self) -> impl Iterator<Item = (VIdx, VIdx)> {
        self.nw.edges().map(|x| (x.tail(), x.head()))
    }
}
