use crate::graphs::extended::GraphExtended;
use crate::graphs::visualization::dot::NodeSettings;
use crate::networks::TrNw;
use crate::networks::com_by_od_st_nw::visualization::dot::DotComOdStNw;
use crate::networks::com_by_od_st_nw::{edge_data::ComOdStDe, vertex_data::ComOdStDv};
use crate::{Problem, Variant};

pub type ComOdStNw<'a, V: Variant> = GraphExtended<'a, TrNw<V>, ComOdStDv, ComOdStDe<V>>;

impl<'a, V: Variant> ComOdStNw<'a, V> {
    pub fn as_dot_graph(
        &'a self,
        p: &'a Problem<V>,
        transport_settings: Option<NodeSettings>,
        source_settings: Option<NodeSettings>,
        sink_settings: Option<NodeSettings>,
    ) -> DotComOdStNw<'a, V> {
        DotComOdStNw::new(p, self, transport_settings, source_settings, sink_settings)
    }
}
