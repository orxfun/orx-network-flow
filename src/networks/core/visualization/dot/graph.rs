use crate::graph::visualization::dot::DotGraph;
use crate::graph::{VIdx, Vertex};
use crate::networks::core::visualization::dot::settings::AonDotGraphSettings;
use crate::networks::core::{AonEdge, AonVertex};
use crate::{CoreNetwork, Graph, Problem, Variant};
use alloc::format;
use alloc::string::{String, ToString};

pub struct CoreDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    network: &'a CoreNetwork<'a, V>,
    settings: AonDotGraphSettings,
}

impl<'a, V: Variant> CoreDotGraph<'a, V> {
    pub fn new(problem: &'a Problem<V>, network: &'a CoreNetwork<'a, V>) -> Self {
        Self::with_settings(problem, network, Default::default())
    }

    pub fn with_settings(
        problem: &'a Problem<V>,
        network: &'a CoreNetwork<'a, V>,
        settings: AonDotGraphSettings,
    ) -> Self {
        Self {
            problem,
            network,
            settings,
        }
    }
}

impl<V: Variant> DotGraph for CoreDotGraph<'_, V> {
    type V = AonVertex;

    type E = AonEdge;

    fn graph(&self) -> &Graph<Self::V, Self::E> {
        self.network.graph()
    }

    fn vertex_label(&self, v: VIdx, vertex: &Vertex<Self::V>) -> String {
        let p = self.problem;
        match vertex.data() {
            AonVertex::Transport(t) => {
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
            AonVertex::Transport(_) => self.settings.transport.to_string(),
        }
    }
}
