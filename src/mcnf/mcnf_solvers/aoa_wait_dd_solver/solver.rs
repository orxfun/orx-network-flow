use crate::McnfSolution;
use crate::graphs::{Edge, Graph};
use crate::mcnf::McnfStats;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::capacity::add_capacity_constraints;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::flow_balance::add_flow_balance_constraints;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::obj::objective;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::params::AoaWaitDdMcnfParams;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::sol::create_solution;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::vars::define_vars;
use crate::networks::AoaWaitNw;
#[cfg(feature = "solver-lp-solvers")]
use crate::utils::math_model::lp_solvers_model_to_problem;
use crate::{Variant, mcnf::mcnf_solvers::aoa_wait_dd_solver::vars::DdVars};
use alloc::string::{String, ToString};
use good_lp::{Solver, SolverModel};
#[cfg(feature = "solver-lp-solvers")]
use lp_solvers::lp_format::LpProblem;

pub struct AoaWaitDdMcnfSolver<'a, V: Variant, S: Solver> {
    nw: &'a AoaWaitNw<'a, V>,
    params: AoaWaitDdMcnfParams,
    dd_vars: DdVars<'a, V>,
    model: S::Model,
}

impl<'a, V: Variant, S: Solver> AoaWaitDdMcnfSolver<'a, V, S> {
    pub fn compute_stats(nw: &'a AoaWaitNw<'a, V>, _params: AoaWaitDdMcnfParams) -> McnfStats {
        let graph_stats = nw.stats();

        let dd_count = nw.p().sorted_dd_commodities.keys().count();

        let mut non_bypass_edges = 0usize;
        let mut bypass_edges = 0usize;
        for e in nw.g().edges() {
            match e.data() {
                crate::networks::AoaWaitEdge::Bypass(_) => bypass_edges += 1,
                _ => non_bypass_edges += 1,
            }
        }

        // +1 for the global dummy variable used for excluded (dd, bypass-edge) combinations.
        let num_variables = 1 + dd_count * non_bypass_edges + bypass_edges;

        // One flow-balance per (dd, vertex) + one capacity per transport arc.
        let num_constraints = dd_count * nw.g().v() + nw.transport_arcs().count();

        McnfStats {
            graph_stats,
            num_variables,
            num_constraints,
        }
    }

    pub fn build(nw: &'a AoaWaitNw<'a, V>, params: AoaWaitDdMcnfParams, solver: S) -> Self {
        let (pr_vars, dd_vars) = define_vars(nw);
        let obj = objective(nw, &dd_vars);

        let mut model = pr_vars.minimise(obj).using(solver);

        add_flow_balance_constraints::<_, S>(nw, &dd_vars, &mut model);
        add_capacity_constraints::<_, S>(nw, &dd_vars, &mut model);

        Self {
            nw,
            params,
            dd_vars,
            model,
        }
    }

    #[cfg(feature = "solver-lp-solvers")]
    pub fn stats(&self) -> McnfStats {
        let graph_stats = self.nw.stats();

        // SAFETY: The model type is produced by good_lp's lp-solvers backend and
        // is layout-compatible with lp_solvers::problem::Problem.
        let p = unsafe { lp_solvers_model_to_problem::<S>(&self.model) };
        let num_variables = p.variables().count();
        let num_constraints = p.constraints().count();

        McnfStats {
            graph_stats,
            num_variables,
            num_constraints,
        }
    }

    #[cfg(all(feature = "solver-lp-solvers", feature = "std"))]
    pub fn display_lp(&self) {
        use crate::utils::math_model::lp_solvers_model_to_problem;
        use lp_solvers::lp_format::LpProblem;
        use std::println;

        let p = unsafe { lp_solvers_model_to_problem::<S>(&self.model) };
        println!("{}", p.display_lp());
    }

    #[cfg(all(feature = "solver-lp-solvers", feature = "std"))]
    pub fn export_lp(&self, lp_path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        use crate::utils::math_model::lp_solvers_model_to_lp_file;
        unsafe { lp_solvers_model_to_lp_file::<S, _>(&self.model, lp_path) }
    }

    pub fn solve(self) -> Result<McnfSolution<V>, String> {
        let solution = self.model.solve().map_err(|e| e.to_string());
        solution.map(|x| create_solution::<_, S>(&self.nw, &self.params, &self.dd_vars, &x))
    }
}
