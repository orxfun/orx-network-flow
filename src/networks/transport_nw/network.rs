use crate::networks::transport_nw::visualization::dot::{TransportNwDot, TransportNwDotSettings};
use crate::networks::transport_nw::{edge::TrNwEdge, vertex::TrNwVertex};
use crate::{Graph, Problem, Variant};

#[derive(derive_new::new)]
pub struct TransportNw<'a, V: Variant> {
    p: &'a Problem<V>,
    graph: Graph<TrNwVertex, TrNwEdge>,
}

impl<'a, V: Variant> TransportNw<'a, V> {
    pub fn graph(&self) -> &Graph<TrNwVertex, TrNwEdge> {
        &self.graph
    }

    // visualization

    pub fn dot(&'a self, custom_settings: Option<TransportNwDotSettings>) -> TransportNwDot<'a, V> {
        match custom_settings {
            Some(settings) => TransportNwDot::with_settings(self.p, self, settings),
            None => TransportNwDot::new(self.p, self),
        }
    }
}
