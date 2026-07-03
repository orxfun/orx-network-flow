use crate::serialization::{
    FormCommodity, FormGeographicSpace, FormLostRevenueItem, FormTransport, ProblemInput,
};
use orx_network_flow::{ProblemBuilder, Variant};

/// Demo variant matching shared_problem.rs
#[derive(Clone, Copy, Default)]
pub struct MyVariant;

impl Variant for MyVariant {
    type S = String;
    type K = usize;
    type W = String;
    type V = usize;
    type T = usize;
    type F = u64;
    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}

/// Build problem from form input
pub fn build_problem_from_input(
    input: ProblemInput,
) -> Result<orx_network_flow::Problem<MyVariant>, String> {
    if input.spaces.is_empty() {
        return Err("At least one geographic space is required".into());
    }

    let builder: ProblemBuilder<MyVariant, _> = ProblemBuilder::new();

    // Add geographic spaces - this changes the builder state to DefiningProblem
    let spaces_iter = input
        .spaces
        .iter()
        .map(|s| (s.name.clone(), s.latitude, s.longitude));
    let builder = builder.with_geographic_spaces(spaces_iter);

    // Now we can call finish() since we're in DefiningProblem state
    Ok(builder.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_empty_problem() {
        let input = ProblemInput {
            spaces: vec![],
            commodities: vec![],
            transports: vec![],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_simple_problem() {
        let input = ProblemInput {
            spaces: vec![
                FormGeographicSpace {
                    name: "A".into(),
                    latitude: 0.0,
                    longitude: 0.0,
                },
                FormGeographicSpace {
                    name: "B".into(),
                    latitude: 1.0,
                    longitude: 1.0,
                },
            ],
            commodities: vec![],
            transports: vec![],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_ok());
    }
}
