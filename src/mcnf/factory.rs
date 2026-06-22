use super::mcnf_solvers::edge_wait_ro_solver::{EdgeWaitRoMcnfParams, EdgeWaitRoMcnfSolver};
use super::mcnf_solvers::space_time_ro_solver::{SpaceTimeRoMcnfParams, SpaceTimeRoMcnfSolver};
use crate::{
    Variant,
    networks::{ConnWaitNw, SpaceTimeNw},
};
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

    pub fn space_time_ro<'a, V, S>(
        nw: &'a SpaceTimeNw<'a, V>,
        params: SpaceTimeRoMcnfParams,
        solver: S,
    ) -> SpaceTimeRoMcnfSolver<'a, V, S>
    where
        V: Variant,
        S: Solver,
    {
        SpaceTimeRoMcnfSolver::build(nw, params, solver)
    }
}
