use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::{AoaWaitDdMcnfParams, AoaWaitDdMcnfSolver};
use crate::networks::AoaWaitNwSettings;
use crate::{ProblemBuilder, Variant};
#[cfg(feature = "solver-lp-solvers")]
use alloc::string::ToString;
#[cfg(feature = "solver-lp-solvers")]
use good_lp::LpSolver;
#[cfg(feature = "solver-lp-solvers")]
use lp_solvers::solvers::Cplex;

#[derive(Clone, Copy, Default)]
struct TestVariant;

impl Variant for TestVariant {
    type S = &'static str;
    type K = usize;
    type W = &'static str;
    type V = usize;
    type T = usize;
    type F = u64;
    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}

#[cfg(feature = "solver-lp-solvers")]
#[test]
fn precomputed_stats_match_built_model_stats() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "X", "B"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 7);
    builder.push_transport(0, 0, "veh", "A", 1_i64, "X", 2_i64, 100);
    builder.push_transport(1, 1, "veh", "X", 3_i64, "B", 4_i64, 100);

    let p = builder.finish();
    let nw = p.construct_aoa_wait_nw(AoaWaitNwSettings {
        add_bypass_edges: true,
    });

    let precomputed = AoaWaitDdMcnfSolver::<_, LpSolver<Cplex>>::compute_stats(
        &nw,
        AoaWaitDdMcnfParams::default(),
    );

    let solver = LpSolver(Cplex::with_command("cplex".to_string()));
    let built = AoaWaitDdMcnfSolver::build(&nw, AoaWaitDdMcnfParams::default(), solver);
    let from_model = built.stats();

    assert_eq!(precomputed, from_model);
}
