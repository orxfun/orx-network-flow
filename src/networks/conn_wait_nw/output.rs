use crate::graphs::{EIdx, Edge, Graph, VecEdge, Vertex};
use crate::networks::ConnWaitNw;
use crate::networks::conn_wait_nw::ConnWaitEdge;
use crate::{Solution, Time, Variant};

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

    let tail = |e: EIdx| g.vertex(g.edge(e).tail());
    let tail_t = |e: EIdx| p.transport_by_idx(tail(e).data().get_t().expect("t"));
    let edge_cost = |e: EIdx, data: &ConnWaitEdge| match data {
        ConnWaitEdge::Connect | ConnWaitEdge::Exit => tail_t(e).duration(),
        _ => Time::zero(),
    };
    let edge_flow = |e: EIdx, data: &ConnWaitEdge| match data {
        ConnWaitEdge::Bypass(_) => Default::default(),
        _ => edge_flows[e],
    };

    builder.finish()
}
