use crate::{Problem, Variant, graphs::core::GraphCore};

pub struct ConnNw<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) g: GraphCore<(), ()>,
}
