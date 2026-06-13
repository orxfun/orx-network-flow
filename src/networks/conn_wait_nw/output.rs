use crate::graphs::VecEdge;
use crate::networks::ConnWaitNw;
use crate::{Solution, Variant};

pub struct Output<V: Variant> {
    pub edge_flows: VecEdge<V::F>,
    pub solution: Solution<V>,
}

impl<V: Variant> Output<V> {
    pub fn create(nw: &ConnWaitNw<'_, V>, edge_flows: VecEdge<V::F>) -> Self {
        let solution = create_solution(nw, edge_flows.clone());
        Self {
            edge_flows,
            solution,
        }
    }
}

fn create_solution<V: Variant>(
    nw: &ConnWaitNw<'_, V>,
    mut edge_flows: VecEdge<V::F>,
) -> Solution<V> {
    let mut builder = Solution::builder(12);
    let b = &mut builder;
    let (p, g) = (nw.p, &nw.g);

    //

    builder.finish()
}
