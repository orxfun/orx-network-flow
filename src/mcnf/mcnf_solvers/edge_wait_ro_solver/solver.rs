use crate::Variant;
use crate::mcnf::McnfSol;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::obj::objective;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::params::EdgeWaitRoMcnfParams;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::vars::define_vars;
use crate::networks::ConnWaitNw;
use good_lp::Solver;

#[derive(Default, Clone, Copy, Debug)]
pub struct EdgeWaitRoMcnfSolver;

impl EdgeWaitRoMcnfSolver {
    pub fn solve_with_solver<V: Variant, S: Solver>(
        &mut self,
        nw: &ConnWaitNw<'_, V>,
        params: &EdgeWaitRoMcnfParams,
        solver: S,
    ) -> McnfSol<V> {
        let (pr_vars, ro_vars) = define_vars(nw);
        let obj = objective(nw, &ro_vars);

        let mut model = pr_vars.minimise(obj).using(solver);

        todo!()
    }
}
