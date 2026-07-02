use super::mcnf_solvers::aoa_wait_dd_solver::{AoaWaitDdMcnfParams, AoaWaitDdMcnfSolver};
use super::mcnf_solvers::aoa_wait_ro_solver::{AoaWaitRoMcnfParams, AoaWaitRoMcnfSolver};
use super::mcnf_solvers::aon_wait_dd_solver::{AonWaitDdMcnfParams, AonWaitDdMcnfSolver};
use super::mcnf_solvers::aon_wait_ro_solver::{AonWaitRoMcnfParams, AonWaitRoMcnfSolver};
use crate::Variant;
use crate::mcnf::McnfStats;
use crate::networks::{AoaWaitNw, AonWaitNw};
use good_lp::Solver;

#[cfg(feature = "solver-microlp")]
type StatsSolver =
    fn(good_lp::variable::UnsolvedProblem) -> good_lp::solvers::microlp::MicroLpProblem;

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

    #[cfg(feature = "solver-microlp")]
    pub fn aon_wait_ro_stats<'a, V>(
        nw: &'a AonWaitNw<'a, V>,
        params: AonWaitRoMcnfParams,
    ) -> McnfStats
    where
        V: Variant,
    {
        AonWaitRoMcnfSolver::<V, StatsSolver>::compute_stats(nw, params)
    }

    pub fn aon_wait_dd<'a, V, S>(
        nw: &'a AonWaitNw<'a, V>,
        params: AonWaitDdMcnfParams,
        solver: S,
    ) -> AonWaitDdMcnfSolver<'a, V, S>
    where
        V: Variant,
        S: Solver,
    {
        AonWaitDdMcnfSolver::build(nw, params, solver)
    }

    #[cfg(feature = "solver-microlp")]
    pub fn aon_wait_dd_stats<'a, V>(
        nw: &'a AonWaitNw<'a, V>,
        params: AonWaitDdMcnfParams,
    ) -> McnfStats
    where
        V: Variant,
    {
        AonWaitDdMcnfSolver::<V, StatsSolver>::compute_stats(nw, params)
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

    #[cfg(feature = "solver-microlp")]
    pub fn aoa_wait_ro_stats<'a, V>(
        nw: &'a AoaWaitNw<'a, V>,
        params: AoaWaitRoMcnfParams,
    ) -> McnfStats
    where
        V: Variant,
    {
        AoaWaitRoMcnfSolver::<V, StatsSolver>::compute_stats(nw, params)
    }

    pub fn aoa_wait_dd<'a, V, S>(
        nw: &'a AoaWaitNw<'a, V>,
        params: AoaWaitDdMcnfParams,
        solver: S,
    ) -> AoaWaitDdMcnfSolver<'a, V, S>
    where
        V: Variant,
        S: Solver,
    {
        AoaWaitDdMcnfSolver::build(nw, params, solver)
    }

    #[cfg(feature = "solver-microlp")]
    pub fn aoa_wait_dd_stats<'a, V>(
        nw: &'a AoaWaitNw<'a, V>,
        params: AoaWaitDdMcnfParams,
    ) -> McnfStats
    where
        V: Variant,
    {
        AoaWaitDdMcnfSolver::<V, StatsSolver>::compute_stats(nw, params)
    }
}
