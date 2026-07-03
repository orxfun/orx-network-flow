use crate::problem_builder::{MyVariant, build_problem_from_input};
use crate::serialization::{McnfStatsResponse, NetworkChoice, ProblemInput};
use orx_network_flow::Problem;

/// Solve network from form input
pub fn solve_network_from_input(
    input: &ProblemInput,
    network_choice: &NetworkChoice,
) -> Result<McnfStatsResponse, String> {
    // Build problem from input
    let problem = build_problem_from_input(input.clone())?;

    // Solve network
    solve_network(&problem, network_choice)
}

/// Solve network with specified configuration
pub fn solve_network(
    problem: &Problem<MyVariant>,
    _network_choice: &NetworkChoice,
) -> Result<McnfStatsResponse, String> {
    // For now, compute basic stats from the problem
    // Later: Implement actual AON/AOA network construction and solving

    Ok(McnfStatsResponse {
        num_variables: 0,   // TODO: Calculate from network
        num_constraints: 0, // TODO: Calculate from network
        num_commodities: problem.len_commodities(),
        num_spaces: problem.len_spaces(),
        num_transports: problem.len_transports(),
    })
}
