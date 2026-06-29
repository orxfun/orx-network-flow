use crate::mcnf::mcnf_solvers::aon_wait_dd_solver::{AonWaitDdMcnfParams, AonWaitDdMcnfSolver};
use crate::networks::AonWaitNwSettings;
use crate::{ProblemBuilder, Variant};
use alloc::string::ToString;
use good_lp::LpSolver;
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

#[test]
fn precomputed_stats_match_built_model_stats() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "X", "B"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 7);
    builder.push_transport(0, 0, "veh", "A", 1_i64, "X", 2_i64, 100);
    builder.push_transport(1, 1, "veh", "X", 3_i64, "B", 4_i64, 100);

    let p = builder.finish();
    let nw = p.construct_aon_wait_nw(AonWaitNwSettings {
        add_bypass_edges: true,
    });

    let precomputed = AonWaitDdMcnfSolver::<_, LpSolver<Cplex>>::compute_stats(
        &nw,
        AonWaitDdMcnfParams::default(),
    );

    let solver = LpSolver(Cplex::with_command("cplex".to_string()));
    let built = AonWaitDdMcnfSolver::build(&nw, AonWaitDdMcnfParams::default(), solver);
    let from_model = built.stats();

    assert_eq!(precomputed, from_model);
}
