use crate::graphs::core::GraphCore;
use crate::graphs::visualization::dot::NodeSettings;
use crate::networks::com_nw_by_od_st::visualization::dot::DotComOdStNw;
use crate::networks::com_nw_by_od_st::{edge_data::ComOdStDe, vertex_data::ComOdStDv};
use crate::{Problem, Variant};

pub type ComOdStNw = GraphCore<ComOdStDe, ComOdStDv>;

impl ComOdStNw {
    pub fn as_dot_graph<'a, V: Variant>(
        &'a self,
        p: &'a Problem<V>,
        node_settings: Option<NodeSettings>,
    ) -> DotComOdStNw<'a, V> {
        // DotComOdStNw::new(p, self, node_settings)
        todo!()
    }
}
