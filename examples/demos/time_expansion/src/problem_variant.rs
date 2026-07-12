use orx_network_flow::{DefiningProblem, DefiningSpaces, Problem, ProblemBuilder, Variant};

#[derive(Clone, Copy, Default)]
pub struct PrVar;

impl Variant for PrVar {
    type S = String;

    type K = String;

    type W = String;

    type V = usize;

    type T = usize;

    type F = u64;

    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}

pub type PrBuilderSpaces = ProblemBuilder<PrVar, DefiningSpaces>;

pub type PrBuilder = ProblemBuilder<PrVar, DefiningProblem>;

pub type Pr = Problem<PrVar>;
