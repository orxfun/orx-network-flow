use crate::problem::{Problem, ProblemBuilder};

#[test]
fn build_problem() {
    let mut builder: ProblemBuilder<&str> = ProblemBuilder::new();

    builder.push_commodity("AMS", 7u32, "BRU", 10u32);
    builder.push_commodity("AMS", 8u32, "LEJ", 20u32);

    let problem = builder.finish();
}
