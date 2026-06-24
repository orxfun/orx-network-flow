use crate::mcnf::mcnf_solvers::aoa_wait_ro_solver::vars::RoVars;
use crate::networks::AoaWaitNw;
use crate::{FlowUnit, Variant};
use alloc::format;
use good_lp::{Expression, Solver, SolverModel, constraint};

pub fn add_capacity_constraints<'a, V: Variant, S: Solver>(
    nw: &AoaWaitNw<'a, V>,
    ro_vars: &RoVars<'a, V>,
    model: &mut S::Model,
) {
    let named = cfg!(debug_assertions);
    let p = nw.p();

    for (t, e) in nw.transport_arcs() {
        let capacity = p.transport_by_idx(t).capacity().into_f64();

        let mut total_flow = Expression::default();
        for (_, vars) in ro_vars.iter() {
            total_flow.add_mul(1, vars[e]);
        }

        let constraint = constraint!(total_flow <= capacity);

        let constraint = match named {
            false => constraint,
            true => constraint.set_name(format!("cap_{t}")),
        };

        model.add_constraint(constraint);
    }
}
