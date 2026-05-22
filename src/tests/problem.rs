use crate::problem::Problem;

#[test]
fn build_problem() {
    let mut problem: Problem<&str> = Problem::new();

    problem.push_commodity("AMS", 7u32, "BRU", 10u32);
    problem.push_commodity("AMS", 8u32, "LEJ", 20u32);
}
