use crate::problem_builder::MyVariant;
use crate::serialization::{McnfResponse, NetworkChoice, ProblemInput};
use orx_network_flow::McnfSolver;
use orx_network_flow::Problem;
use orx_network_flow::networks::{AoaWaitNwSettings, AonWaitNwSettings};
use orx_network_flow::solvers;
use orx_network_flow::{McnfSolution, Variant};

/// Solve network from form input
pub fn solve_network_from_input(
    input: &ProblemInput,
    network_choice: &NetworkChoice,
) -> Result<McnfResponse, String> {
    use crate::problem_builder::build_problem_from_input;

    // Build problem from input
    let problem = build_problem_from_input(input.clone())?;

    // Solve network
    solve_network(&problem, network_choice)
}

/// Solve network with specified configuration
pub fn solve_network(
    problem: &Problem<MyVariant>,
    network_choice: &NetworkChoice,
) -> Result<McnfResponse, String> {
    let network_type = network_choice.network_type.as_str();
    let grouping = network_choice.grouping_strategy.as_str();

    // Validate inputs
    if network_type != "aon" && network_type != "aoa" {
        return Err(format!(
            "Invalid network type: {}. Must be 'aon' or 'aoa'",
            network_type
        ));
    }
    if grouping != "dd" && grouping != "ro" {
        return Err(format!(
            "Invalid grouping strategy: {}. Must be 'dd' or 'ro'",
            grouping
        ));
    }

    // Network construction settings
    let settings = AonWaitNwSettings {
        add_bypass_edges: true,
    };

    // Use microlp solver (pure Rust, works in WASM)
    let solver = solvers::microlp;

    // Dispatch based on network type and grouping
    match (network_type, grouping) {
        ("aon", "dd") => {
            // Construct AON Wait network with DD disaggregation
            let nw = problem.construct_aon_wait_nw(settings);

            // Create solver and get stats
            let mcnf_solver = McnfSolver::aon_wait_dd(&nw, Default::default(), solver);
            let stats = mcnf_solver.stats();

            // Solve the problem
            let solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            // Compute objective value from solution
            let objective_value = compute_objective_value(&solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
            })
        }
        ("aon", "ro") => {
            // Construct AON Wait network with RO disaggregation
            let nw = problem.construct_aon_wait_nw(settings);

            // Create solver and get stats
            let mcnf_solver = McnfSolver::aon_wait_ro(&nw, Default::default(), solver);
            let stats = mcnf_solver.stats();

            // Solve the problem
            let solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            // Compute objective value from solution
            let objective_value = compute_objective_value(&solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
            })
        }
        ("aoa", "dd") => {
            // Construct AOA Wait network with DD disaggregation
            let aoa_settings = AoaWaitNwSettings {
                add_bypass_edges: true,
            };
            let nw = problem.construct_aoa_wait_nw(aoa_settings);

            // Create solver and get stats
            let mcnf_solver = McnfSolver::aoa_wait_dd(&nw, Default::default(), solver);
            let stats = mcnf_solver.stats();

            // Solve the problem
            let solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            // Compute objective value from solution
            let objective_value = compute_objective_value(&solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
            })
        }
        ("aoa", "ro") => {
            // Construct AOA Wait network with RO disaggregation
            let aoa_settings = AoaWaitNwSettings {
                add_bypass_edges: true,
            };
            let nw = problem.construct_aoa_wait_nw(aoa_settings);

            // Create solver and get stats
            let mcnf_solver = McnfSolver::aoa_wait_ro(&nw, Default::default(), solver);
            let stats = mcnf_solver.stats();

            // Solve the problem
            let solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            // Compute objective value from solution
            let objective_value = compute_objective_value(&solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
            })
        }
        _ => Err("Unreachable: network type and grouping should have been validated".into()),
    }
}

/// Compute objective value from solution by summing flows
fn compute_objective_value<V: Variant>(solution: &McnfSolution<V>) -> f64 {
    let mut total_flow = 0.0;

    // Iterate through all transport loads and sum the flows
    for loads in solution.transport_loads().iter() {
        for _load in loads {
            // Sum all flows across all transports
            // _load.load is of type V::F, typically u64
            // For now, we count each commodity load as 1 unit
            total_flow += 1.0;
        }
    }

    total_flow
}
