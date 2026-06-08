use crate::graphs::core::GraphCore;
use crate::networks::transport_nw::{edge_data::TrDe, vertex_data::TrDv};
use crate::{Problem, Variant};

pub struct TrNw<'a, V: Variant> {
    p: &'a Problem<V>,
    graph: GraphCore<TrDv, TrDe<V>>,
}

impl<'a, V: Variant> TrNw<'a, V> {
    pub(super) fn new(p: &'a Problem<V>, graph: GraphCore<TrDv, TrDe<V>>) -> Self {
        Self { p, graph }
    }
}
