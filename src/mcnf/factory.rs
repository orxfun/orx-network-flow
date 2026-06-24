use super::mcnf_solvers::aoa_wait_ro_solver::{AoaWaitRoMcnfParams, AoaWaitRoMcnfSolver};
use super::mcnf_solvers::aon_wait_ro_solver::{AonWaitRoMcnfParams, AonWaitRoMcnfSolver};
use crate::Variant;
use crate::mcnf::McnfStats;
use crate::networks::{AoaWaitNw, AonWaitNw};
use good_lp::Solver;

type StatsSolver = good_lp::solvers::lp_solvers::LpSolver<lp_solvers::solvers::Cplex>;

pub struct McnfSolver;

impl McnfSolver {
    // aon - wait

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

    pub fn aon_wait_ro_stats<'a, V>(
        nw: &'a AonWaitNw<'a, V>,
        params: AonWaitRoMcnfParams,
    ) -> McnfStats
    where
        V: Variant,
    {
        AonWaitRoMcnfSolver::<V, StatsSolver>::compute_stats(nw, params)
    }

    // aoa - wait

    pub fn aoa_wait_ro<'a, V, S>(
        nw: &'a AoaWaitNw<'a, V>,
        params: AoaWaitRoMcnfParams,
        solver: S,
    ) -> AoaWaitRoMcnfSolver<'a, V, S>
    where
        V: Variant,
        S: Solver,
    {
        AoaWaitRoMcnfSolver::build(nw, params, solver)
    }

    pub fn aoa_wait_ro_stats<'a, V>(
        nw: &'a AoaWaitNw<'a, V>,
        params: AoaWaitRoMcnfParams,
    ) -> McnfStats
    where
        V: Variant,
    {
        AoaWaitRoMcnfSolver::<V, StatsSolver>::compute_stats(nw, params)
    }
}
