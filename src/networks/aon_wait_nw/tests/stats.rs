use crate::networks::{AonWaitNw, AonWaitNwSettings};
use crate::{ProblemBuilder, Variant};

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
fn compute_stats_matches_stats_after_construction() {
    let mut builder: ProblemBuilder<TestVariant, _> =
        ProblemBuilder::new().with_basic_spaces(["A", "X", "B"]);

    builder.push_commodity(0, "A", 0_i64, "B", 10_i64, 7);
    builder.push_transport(0, 0, "veh", "A", 1_i64, "X", 2_i64, 100);
    builder.push_transport(1, 1, "veh", "X", 3_i64, "B", 4_i64, 100);

    let p = builder.finish();
    let settings = AonWaitNwSettings {
        add_bypass_edges: true,
    };

    let computed = AonWaitNw::compute_stats(&p, settings);
    let nw = p.construct_aon_wait_nw(settings);

    assert_eq!(computed, nw.stats());
}
