use crate::mcnf::mcnf_solvers::aon_wait_dd_solver::vars::DdVars;
use crate::networks::AonWaitNw;
use crate::{FlowUnit, Variant};
use alloc::format;
use good_lp::{Expression, Solver, SolverModel, constraint};

pub fn add_capacity_constraints<'a, V: Variant, S: Solver>(
    nw: &AonWaitNw<'a, V>,
    dd_vars: &DdVars<'a, V>,
    model: &mut S::Model,
) {
    let named = cfg!(debug_assertions);
    let p = nw.p();

    for (t, edges) in nw.transport_edges() {
        if edges.is_empty() {
            continue;
        }

        let capacity = p.transport_by_idx(t).capacity().into_f64();

        let mut total_flow = Expression::default();
        for &e in edges {
            for (_, vars) in dd_vars.iter() {
                total_flow.add_mul(1, vars[e]);
            }
        }

        let constraint = constraint!(total_flow <= capacity);

        let constraint = match named {
            false => constraint,
            true => constraint.set_name(format!("cap_{t}")),
        };

        model.add_constraint(constraint);
    }
}
