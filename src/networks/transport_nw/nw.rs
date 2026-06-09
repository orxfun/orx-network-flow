use crate::graphs::core::GraphCore;
use crate::graphs::visualization::dot::NodeSettings;
use crate::networks::transport_nw::visualization::dot::DotTrNw;
use crate::networks::transport_nw::{edge_data::TrDe, vertex_data::TrDv};
use crate::{Problem, Variant};

pub type TrNw<V> = GraphCore<TrDv, TrDe<V>>;

impl<V: Variant> GraphCore<TrDv, TrDe<V>> {
    pub fn as_dot_graph<'a>(
        &'a self,
        p: &'a Problem<V>,
        node_settings: Option<NodeSettings>,
    ) -> DotTrNw<'a, V> {
        DotTrNw::new(p, self, node_settings)
    }
}
