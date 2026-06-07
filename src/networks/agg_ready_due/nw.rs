use crate::{CoreNw, Graph, Problem, Variant};

#[derive(derive_new::new)]
pub struct AggReadyDueNw<'a, V: Variant> {
    p: &'a Problem<V>,
    core: &'a CoreNw<'a, V>,
}
