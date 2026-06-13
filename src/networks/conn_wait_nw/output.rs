use crate::graphs::VecEdge;
use crate::networks::ConnWaitNw;
use crate::{Solution, Variant};

pub struct Output<V: Variant> {
    edge_flows: VecEdge<V::F>,
    solution: Solution<V>,
}

impl<V: Variant> Output<V> {
    pub fn create(edge_flows: VecEdge<V::F>) -> Self {
        todo!()
    }
}

fn create_solution<V: Variant>(
    nw: &ConnWaitNw<'_, V>,
    mut edge_flows: VecEdge<V::F>,
) -> Solution<V> {
    let mut builder = Solution::builder(12);
    let b = &mut builder;

    builder.finish()
}
