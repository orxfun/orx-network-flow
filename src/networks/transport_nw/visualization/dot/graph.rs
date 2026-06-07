use crate::graph::visualization::dot::DotGraph;
use crate::graph::{VIdx, Vertex};
use crate::networks::transport_nw::visualization::dot::settings::TransportNwDotSettings;
use crate::networks::transport_nw::{TrNwEdge, TrNwVertex};
use crate::{Graph, Problem, TransportNw, Variant};
use alloc::format;
use alloc::string::{String, ToString};

pub struct TransportNwDot<'a, V: Variant> {
    problem: &'a Problem<V>,
    network: &'a TransportNw<'a, V>,
    settings: TransportNwDotSettings,
}

impl<'a, V: Variant> TransportNwDot<'a, V> {
    pub fn new(problem: &'a Problem<V>, network: &'a TransportNw<'a, V>) -> Self {
        Self::with_settings(problem, network, Default::default())
    }

    pub fn with_settings(
        problem: &'a Problem<V>,
        network: &'a TransportNw<'a, V>,
        settings: TransportNwDotSettings,
    ) -> Self {
        Self {
            problem,
            network,
            settings,
        }
    }
}

impl<V: Variant> DotGraph for TransportNwDot<'_, V> {
    type V = TrNwVertex;

    type E = TrNwEdge;

    fn graph(&self) -> &Graph<Self::V, Self::E> {
        self.network.graph()
    }

    fn vertex_label(&self, v: VIdx, vertex: &Vertex<Self::V>) -> String {
        let p = self.problem;
        match vertex.data() {
            TrNwVertex::Transport(t) => {
                let transport = p.transport_by_idx(*t);
                let ori = p.space_key(transport.origin().space());
                let des = p.space_key(transport.destination().space());
                let dt = transport.origin().time();
                let at = transport.destination().time();
                format!("{}\n{}-{}\n{}-{}", v, ori, des, dt, at)
            }
        }
    }

    fn vertex_tooltip(&self, _: VIdx, _: &Vertex<Self::V>) -> Option<String> {
        None
    }

    fn vertex_settings(&self, _: VIdx, vertex: &Vertex<Self::V>) -> String {
        match vertex.data() {
            TrNwVertex::Transport(_) => self.settings.transport.to_string(),
        }
    }
}
