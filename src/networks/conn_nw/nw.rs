use crate::graphs::{core::GraphCore, visualization::dot::NodeSettings};
use crate::networks::conn_nw::visualization::dot::DotConnNw;
use crate::{Problem, Variant};

pub struct ConnNw<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) g: GraphCore<(), ()>,
}

impl<'a, V> ConnNw<'a, V>
where
    V: Variant,
{
    pub fn construct(p: &'a Problem<V>) -> Self {
        todo!()
    }

    pub fn as_dot_graph(&'a self, transport_settings: Option<NodeSettings>) -> DotConnNw<'a, V> {
        DotConnNw::new(self, transport_settings)
    }
}
