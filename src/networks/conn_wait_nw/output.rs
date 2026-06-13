use crate::{Solution, Variant, graphs::VecEdge};

pub struct Output<V: Variant> {
    edge_flows: VecEdge<V::F>,
    solution: Solution<V>,
}
