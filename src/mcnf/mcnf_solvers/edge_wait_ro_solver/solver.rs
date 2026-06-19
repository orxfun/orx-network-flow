use std::dbg;

use crate::Variant;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::capacity::add_capacity_constraints;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::flow_balance::add_flow_balance_constraints;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::obj::objective;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::params::EdgeWaitRoMcnfParams;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::vars::define_vars;
use crate::networks::ConnWaitNw;
use good_lp::{Solver, SolverModel};

pub struct EdgeWaitRoMcnfSolver<'a, V: Variant, S: Solver> {
    nw: &'a ConnWaitNw<'a, V>,
    params: EdgeWaitRoMcnfParams,
    model: S::Model,
}

impl<'a, V: Variant, S: Solver> EdgeWaitRoMcnfSolver<'a, V, S> {
    pub fn build(nw: &'a ConnWaitNw<'a, V>, params: EdgeWaitRoMcnfParams, solver: S) -> Self {
        let (pr_vars, ro_vars) = define_vars(nw);
        let obj = objective(nw, &ro_vars);

        let mut model = pr_vars.minimise(obj).using(solver);

        add_flow_balance_constraints::<_, S>(nw, &ro_vars, &mut model);
        add_capacity_constraints::<_, S>(nw, &ro_vars, &mut model);

        Self { nw, params, model }
    }

    #[cfg(feature = "std")]
    pub fn display_lp(&self, lp_path: impl AsRef<std::path::Path>) {
        use crate::utils::math_model::lp_solvers_model_to_problem;
        use lp_solvers::lp_format::LpProblem;
        let p = unsafe { lp_solvers_model_to_problem::<S>(&self.model) };
        p.display_lp();
    }

    #[cfg(feature = "std")]
    pub fn export_lp(&self, lp_path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        use crate::utils::math_model::lp_solvers_model_to_lp_file;
        unsafe { lp_solvers_model_to_lp_file::<S, _>(&self.model, lp_path) }
    }

    pub fn solve(self) {
        let solution = self.model.solve();
        let a = solution.unwrap();
        let b = a;
    }
}
