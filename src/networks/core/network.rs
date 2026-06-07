use crate::networks::core::visualization::dot::{AonDotGraphSettings, CoreDotGraph};
use crate::networks::core::{edge::AonEdge, vertex::AonVertex};
use crate::{Graph, Problem, Variant};

#[derive(derive_new::new)]
pub struct CoreNetwork<'a, V: Variant> {
    p: &'a Problem<V>,
    graph: Graph<AonVertex, AonEdge>,
}

impl<'a, V: Variant> CoreNetwork<'a, V> {
    pub fn graph(&self) -> &Graph<AonVertex, AonEdge> {
        &self.graph
    }

    // visualization

    pub fn dot(&'a self, custom_settings: Option<AonDotGraphSettings>) -> CoreDotGraph<'a, V> {
        match custom_settings {
            Some(settings) => CoreDotGraph::with_settings(self.p, self, settings),
            None => CoreDotGraph::new(self.p, self),
        }
    }
}
