use super::mcnf_solvers::aon_wait_ro_solver::{AonWaitRoMcnfParams, AonWaitRoMcnfSolver};
use super::mcnf_solvers::space_time_ro_solver::{SpaceTimeRoMcnfParams, SpaceTimeRoMcnfSolver};
use crate::{
    Variant,
    networks::{AonWaitNw, SpaceTimeNw},
};
use good_lp::Solver;

pub struct McnfSolver;

impl McnfSolver {
    pub fn aon_wait_ro<'a, V, S>(
        nw: &'a AonWaitNw<'a, V>,
        params: AonWaitRoMcnfParams,
        solver: S,
    ) -> AonWaitRoMcnfSolver<'a, V, S>
    where
        V: Variant,
        S: Solver,
    {
        AonWaitRoMcnfSolver::build(nw, params, solver)
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
