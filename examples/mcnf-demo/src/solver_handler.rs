use crate::problem_builder::MyVariant;
use crate::serialization::{McnfStatsResponse, NetworkChoice, ProblemInput};
use orx_network_flow::McnfSolver;
use orx_network_flow::Problem;
use orx_network_flow::networks::{AoaWaitNwSettings, AonWaitNwSettings};
use orx_network_flow::solvers;

/// Solve network from form input
pub fn solve_network_from_input(
    input: &ProblemInput,
    network_choice: &NetworkChoice,
) -> Result<McnfStatsResponse, String> {
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
) -> Result<McnfStatsResponse, String> {
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
            let _solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            Ok(McnfStatsResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: None,
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
            let _solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            Ok(McnfStatsResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: None,
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
            let _solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            Ok(McnfStatsResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: None,
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
            let _solution = mcnf_solver
                .solve()
                .map_err(|e| format!("Solver error: {}", e))?;

            Ok(McnfStatsResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: None,
                status: Some("optimal".to_string()),
            })
        }
        _ => Err("Unreachable: network type and grouping should have been validated".into()),
    }
}
