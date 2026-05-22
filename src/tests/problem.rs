use crate::problem::{Problem, ProblemBuilder};

#[test]
fn build_problem() {
    let mut builder: ProblemBuilder<&str, usize> = ProblemBuilder::new();

    builder.push_commodity(0, "AMS", 7u32, "BRU", 10u32);
    builder.push_commodity(1, "AMS", 8u32, "LEJ", 20u32);

    let problem = builder.finish();
}
