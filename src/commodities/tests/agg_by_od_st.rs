use crate::commodities::CommoditiesByOdSt;
use crate::space_time::{SpaceTime, SpaceTimeOd};
use crate::{Problem, ProblemBuilder, Variant};
use alloc::vec;
use alloc::vec::Vec;

struct TestVariant;

impl Variant for TestVariant {
    type S = &'static str;
    type K = u32;
    type W = &'static str;
    type V = u32;
    type T = u32;
    type F = u64;
    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}

fn od(
    problem: &Problem<TestVariant>,
    ori: &'static str,
    rt: i64,
    des: &'static str,
    dt: i64,
) -> SpaceTimeOd {
    let ori = problem.space_idx(&ori).expect("origin space should exist");
    let des = problem
        .space_idx(&des)
        .expect("destination space should exist");
    SpaceTimeOd::new(SpaceTime::new(ori, rt), SpaceTime::new(des, dt))
}

#[test]
fn create_groups_commodities_by_origin_destination_and_time() {
    let mut builder = ProblemBuilder::<TestVariant, _>::new().with_basic_spaces(["A", "B", "C"]);

    builder.push_commodity(100, "A", 0_i64, "B", 10_i64, 10);
    builder.push_commodity(101, "A", 0_i64, "B", 10_i64, 20);
    builder.push_commodity(102, "A", 1_i64, "B", 10_i64, 30);
    builder.push_commodity(103, "A", 0_i64, "C", 10_i64, 40);

    let problem = builder.finish();
    let grouped = CommoditiesByOdSt::create(&problem);

    assert_eq!(grouped.len_groups(), 3);

    let c100 = problem.commodity_ind(&100).expect("commodity should exist");
    let c101 = problem.commodity_ind(&101).expect("commodity should exist");
    let c102 = problem.commodity_ind(&102).expect("commodity should exist");
    let c103 = problem.commodity_ind(&103).expect("commodity should exist");

    let a0_b10 = od(&problem, "A", 0_i64, "B", 10_i64);
    let a1_b10 = od(&problem, "A", 1_i64, "B", 10_i64);
    let a0_c10 = od(&problem, "A", 0_i64, "C", 10_i64);

    let a0_b10_group = grouped
        .group(&a0_b10)
        .expect("expected group for A@0 -> B@10");
    let a0_b10_members = a0_b10_group.indices().collect::<Vec<_>>();
    assert_eq!(a0_b10_members, vec![c100, c101]);

    let a1_b10_group = grouped
        .group(&a1_b10)
        .expect("expected group for A@1 -> B@10");
    assert_eq!(a1_b10_group.indices().collect::<Vec<_>>(), vec![c102]);

    let a0_c10_group = grouped
        .group(&a0_c10)
        .expect("expected group for A@0 -> C@10");
    assert_eq!(a0_c10_group.indices().collect::<Vec<_>>(), vec![c103]);
}

#[test]
fn create_returns_empty_groups_when_problem_has_no_commodities() {
    let problem = ProblemBuilder::<TestVariant, _>::new()
        .with_basic_spaces(["A", "B"])
        .finish();

    let grouped = CommoditiesByOdSt::create(&problem);

    assert_eq!(grouped.len_groups(), 0);

    let a0_b10 = od(&problem, "A", 0_i64, "B", 10_i64);
    assert!(grouped.group(&a0_b10).is_none());
}
