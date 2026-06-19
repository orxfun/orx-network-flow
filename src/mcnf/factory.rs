use super::mcnf_solvers::edge_wait_ro_solver::{EdgeWaitRoMcnfParams, EdgeWaitRoMcnfSolver};
use crate::{Variant, networks::ConnWaitNw};
use good_lp::Solver;

pub struct McnfSolver;

impl McnfSolver {
    pub fn edge_wait_ro<'a, V, S>(
        nw: &'a ConnWaitNw<'a, V>,
        params: EdgeWaitRoMcnfParams,
        solver: S,
    ) -> EdgeWaitRoMcnfSolver<'a, V, S>
    where
        V: Variant,
        S: Solver,
    {
        EdgeWaitRoMcnfSolver::build(nw, params, solver)
    }
}
