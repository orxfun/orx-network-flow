use crate::graphs::core::VertexCore;
use crate::graphs::{Graph, Vertex};
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::vars::DdVars;
use crate::networks::{AoaWaitNw, AoaWaitVertex};
use crate::{FlowUnit, Problem, SpaceTime, Variant};
use alloc::{format, string::String};
use good_lp::{Expression, Solver, SolverModel, constraint};

pub fn add_flow_balance_constraints<'a, V: Variant, S: Solver>(
    nw: &AoaWaitNw<'a, V>,
    dd_vars: &DdVars<'a, V>,
    model: &mut S::Model,
) {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), nw.g());

    for (dd, vars) in dd_vars.iter() {
        let dd_commodities = p.sorted_dd_commodities.value_by_key_unc(&dd);
        let total_demand = FlowUnit::sum(dd_commodities.iter().map(|&c| p.commodity_by_idx(c).amount()));

        for vertex in g.vertices() {
            let mut out_minus_in = Expression::default();

            for e in vertex.out_edges() {
                out_minus_in.add_mul(1, vars[e]);
            }

            for e in vertex.in_edges() {
                out_minus_in.add_mul(-1, vars[e]);
            }

            let vertex_st = vertex.data().0;

            let b = if vertex_st == dd {
                -total_demand.into_f64()
            } else {
                let commodities = dd_commodities.iter().map(|&c| p.commodity_by_idx(c));
                let commodities = commodities.filter(|c| c.origin() == vertex_st);
                let supply = commodities.map(|c| c.amount());
                FlowUnit::sum(supply).into_f64()
            };

            let constraint = constraint!(out_minus_in == b);

            let constraint = match named {
                true => constraint.set_name(constraint_name(p, dd, vertex)),
                false => constraint,
            };

            model.add_constraint(constraint);
        }
    }
}

fn constraint_name<V: Variant>(
    p: &Problem<V>,
    dd: SpaceTime,
    vertex: &VertexCore<AoaWaitVertex>,
) -> String {
    let d = p.space_key(dd.space());
    let due = dd.time();
    let st = vertex.data().0;
    let s = p.space_key(st.space());
    format!("fb__{d}_{due}__{s}_{}", st.time())
}
