use crate::graphs::core::VertexCore;
use crate::graphs::{Graph, Vertex};
use crate::mcnf::mcnf_solvers::space_time_ro_solver::vars::RoVars;
use crate::networks::{SpaceTimeNw, SpaceTimeVertex};
use crate::{FlowUnit, Problem, SpaceTime, Variant};
use alloc::{format, string::String};
use good_lp::{Expression, Solver, SolverModel, constraint};

pub fn add_flow_balance_constraints<'a, V: Variant, S: Solver>(
    nw: &SpaceTimeNw<'a, V>,
    ro_vars: &RoVars<'a, V>,
    model: &mut S::Model,
) {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), nw.g());

    for (ro, vars) in ro_vars.iter() {
        let ro_commodities = p.sorted_ro_commodities.value_by_key_unc(&ro);

        for vertex in g.vertices() {
            let mut out_minus_in = Expression::default();

            for e in vertex.out_edges() {
                out_minus_in.add_mul(1, vars[e]);
            }

            for e in vertex.in_edges() {
                out_minus_in.add_mul(-1, vars[e]);
            }

            let vertex_st = vertex.data().0;

            let b = if vertex_st == ro {
                // supply at the ready-origin node
                let commodities = ro_commodities.iter().map(|&c| p.commodity_by_idx(c));
                let demand = commodities.map(|c| c.amount());
                FlowUnit::sum(demand).into_f64()
            } else {
                // demand at a due-destination, filtered to commodities in this RO
                let commodities = ro_commodities.iter().map(|&c| p.commodity_by_idx(c));
                let commodities = commodities.filter(|c| c.destination() == vertex_st);
                let demand = commodities.map(|c| c.amount());
                -FlowUnit::sum(demand).into_f64()
            };

            let constraint = constraint!(out_minus_in == b);

            let constraint = match named {
                true => constraint.set_name(constraint_name(p, ro, vertex)),
                false => constraint,
            };

            model.add_constraint(constraint);
        }
    }
}

fn constraint_name<V: Variant>(
    p: &Problem<V>,
    ro: SpaceTime,
    vertex: &VertexCore<SpaceTimeVertex>,
) -> String {
    let o = p.space_key(ro.space());
    let r = ro.time();
    let st = vertex.data().0;
    let s = p.space_key(st.space());
    format!("fb__{o}_{r}__{s}_{}", st.time())
}
