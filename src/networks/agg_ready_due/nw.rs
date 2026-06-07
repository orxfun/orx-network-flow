use crate::{Graph, Problem, Variant};

#[derive(derive_new::new)]
pub struct AggReadyDueNw<'a, V: Variant> {
    p: &'a Problem<V>,
    // graph: Graph<CoreNwVertex, CoreNwEdge>,
}
