use crate::networks::core::visualization::dot::{CoreNwDot, CoreNwDotSettings};
use crate::networks::core::{edge::CoreNwEdge, vertex::CoreNwVertex};
use crate::{Graph, Problem, Variant};

#[derive(derive_new::new)]
pub struct CoreNw<'a, V: Variant> {
    p: &'a Problem<V>,
    graph: Graph<CoreNwVertex, CoreNwEdge>,
}

impl<'a, V: Variant> CoreNw<'a, V> {
    pub fn graph(&self) -> &Graph<CoreNwVertex, CoreNwEdge> {
        &self.graph
    }

    // visualization

    pub fn dot(&'a self, custom_settings: Option<CoreNwDotSettings>) -> CoreNwDot<'a, V> {
        match custom_settings {
            Some(settings) => CoreNwDot::with_settings(self.p, self, settings),
            None => CoreNwDot::new(self.p, self),
        }
    }
}
