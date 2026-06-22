use crate::graphs::{Edge, Graph, Vertex};
use crate::mcnf::mcnf_solvers::space_time_ro_solver::vars::RoVars;
use crate::networks::SpaceTimeNw;
use crate::{Variant, cost::Cost};
use good_lp::Expression;

pub fn objective<'a, V>(nw: &SpaceTimeNw<'a, V>, ro_vars: &RoVars<'a, V>) -> Expression
where
    V: Variant,
{
    let (p, g) = (nw.p(), nw.g());
    let mut cost = Expression::default();

    let bypass_edges_data = g.edges_slice(nw.bypass_edges_range());
    let bypass_edge_indices = nw.bypass_edges_range().iter();
    for (e, edge) in bypass_edge_indices.zip(bypass_edges_data) {
        let ro = g.vertex(edge.tail()).data().0;
        let vars = ro_vars.vars_of(ro);

        let c = edge.data().get_bypass_c().expect("bypass");
        let coef = p.costs.lost_revenue.cost(c);
        cost.add_mul(coef.into_f64(), vars[e]);
    }

    cost
}
