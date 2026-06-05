use crate::graph::visualization::dot::DotGraph;
use crate::graph::{VIdx, Vertex};
use crate::networks::aon::edge::AonEdge;
use crate::networks::aon::vertex::AonVertex;
use crate::networks::aon::visualization::dot::settings::AonDotGraphSettings;
use crate::{AonNetwork, Graph, Problem, Variant};
use alloc::string::String;

pub struct AonDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    network: &'a AonNetwork,
    settings: AonDotGraphSettings,
}

impl<'a, V: Variant> AonDotGraph<'a, V> {
    pub fn new(problem: &'a Problem<V>, network: &'a AonNetwork) -> Self {
        Self::with_settings(problem, network, Default::default())
    }

    pub fn with_settings(
        problem: &'a Problem<V>,
        network: &'a AonNetwork,
        settings: AonDotGraphSettings,
    ) -> Self {
        Self {
            problem,
            network,
            settings,
        }
    }
}

impl<V: Variant> DotGraph for AonDotGraph<'_, V> {
    type V = AonVertex;

    type E = AonEdge;

    fn graph(&self) -> &Graph<Self::V, Self::E> {
        self.network.graph()
    }

    fn vertex_label(&self, v: VIdx, vertex: &Vertex<Self::V>) -> String {
        todo!()
    }

    fn vertex_settings(&self, v: VIdx, vertex: &Vertex<Self::V>) -> String {
        todo!()
    }
}
