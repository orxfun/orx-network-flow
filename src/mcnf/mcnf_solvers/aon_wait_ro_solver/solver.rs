use crate::McnfSolution;
use crate::mcnf::mcnf_solvers::aon_wait_ro_solver::capacity::add_capacity_constraints;
use crate::mcnf::mcnf_solvers::aon_wait_ro_solver::flow_balance::add_flow_balance_constraints;
use crate::mcnf::mcnf_solvers::aon_wait_ro_solver::obj::objective;
use crate::mcnf::mcnf_solvers::aon_wait_ro_solver::params::AonWaitRoMcnfParams;
use crate::mcnf::mcnf_solvers::aon_wait_ro_solver::sol::create_solution;
use crate::mcnf::mcnf_solvers::aon_wait_ro_solver::vars::define_vars;
use crate::networks::AonWaitNw;
use crate::{Variant, mcnf::mcnf_solvers::aon_wait_ro_solver::vars::RoVars};
use alloc::string::{String, ToString};
use good_lp::{Solver, SolverModel};

pub struct AonWaitRoMcnfSolver<'a, V: Variant, S: Solver> {
    nw: &'a AonWaitNw<'a, V>,
    params: AonWaitRoMcnfParams,
    ro_vars: RoVars<'a, V>,
    model: S::Model,
}

impl<'a, V: Variant, S: Solver> AonWaitRoMcnfSolver<'a, V, S> {
    pub fn build(nw: &'a AonWaitNw<'a, V>, params: AonWaitRoMcnfParams, solver: S) -> Self {
        let (pr_vars, ro_vars) = define_vars(nw);
        let obj = objective(nw, &ro_vars);

        let mut model = pr_vars.minimise(obj).using(solver);

        add_flow_balance_constraints::<_, S>(nw, &ro_vars, &mut model);
        add_capacity_constraints::<_, S>(nw, &ro_vars, &mut model);

        Self {
            nw,
            params,
            ro_vars,
            model,
        }
    }

    #[cfg(feature = "std")]
    pub fn display_lp(&self) {
        use crate::utils::math_model::lp_solvers_model_to_problem;
        use lp_solvers::lp_format::LpProblem;
        use std::println;

        let p = unsafe { lp_solvers_model_to_problem::<S>(&self.model) };
        println!("{}", p.display_lp());
    }

    #[cfg(feature = "std")]
    pub fn export_lp(&self, lp_path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        use crate::utils::math_model::lp_solvers_model_to_lp_file;
        unsafe { lp_solvers_model_to_lp_file::<S, _>(&self.model, lp_path) }
    }

    pub fn solve(self) -> Result<McnfSolution<V>, String> {
        let solution = self.model.solve().map_err(|e| e.to_string());
        solution.map(|x| create_solution::<_, S>(&self.nw, &self.params, &self.ro_vars, &x))
    }
}
