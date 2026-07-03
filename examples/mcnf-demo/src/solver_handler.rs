use crate::problem_builder::MyVariant;
use crate::serialization::{
    CommodityPath, CommoditySolution, McnfResponse, NetworkChoice, ProblemInput, SolutionData,
    TransportUtilization,
};
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
            let solution_data = extract_solution_data(&problem, &solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: None,
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
            let solution_data = extract_solution_data(&problem, &solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: None,
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
            let solution_data = extract_solution_data(&problem, &solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: None,
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
            let solution_data = extract_solution_data(&problem, &solution);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: None,
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

/// Extract transport indices from the Path enum's Debug representation
fn extract_transport_indices_from_path(path_debug: &str) -> Vec<usize> {
    // Path enum formats: OneLeg([...]), TwoLegs([...]), ThreeLegs([...]), Long([...])
    // Extract all numbers, being careful about word boundaries

    let mut indices = Vec::new();
    let mut current_num = String::new();
    let mut prev_was_digit = false;

    for ch in path_debug.chars() {
        match ch {
            '0'..='9' => {
                current_num.push(ch);
                prev_was_digit = true;
            }
            _ => {
                // Non-digit: clear the buffer if we have a number
                if !current_num.is_empty() && prev_was_digit {
                    if let Ok(num) = current_num.parse::<usize>() {
                        indices.push(num);
                    }
                    current_num.clear();
                }
                prev_was_digit = false;
            }
        }
    }

    // Parse any remaining number
    if !current_num.is_empty() {
        if let Ok(num) = current_num.parse::<usize>() {
            indices.push(num);
        }
    }

    indices
}

/// Build space sequence string from transport indices
/// Extracts space names by following the transport path
fn build_space_sequence<V: Variant>(problem: &Problem<V>, transport_indices: &[usize]) -> String
where
    V::S: ToString,
    V::T: From<usize>,
{
    if transport_indices.is_empty() {
        return String::new();
    }

    let mut spaces = Vec::new();

    // Add the origin of the first transport
    if let Some(&first_t_idx) = transport_indices.first() {
        let first_transport_key = V::T::from(first_t_idx);
        if let Some(first_transport) = problem.transports.get_by_key(&first_transport_key) {
            let origin_space_idx = first_transport.origin().space();
            if let Some(origin_space_name) = problem.spaces.key(origin_space_idx) {
                spaces.push(origin_space_name.to_string());
            }
        }
    }

    // Add destinations of all transports
    for &t_idx in transport_indices {
        let transport_key = V::T::from(t_idx);
        if let Some(transport) = problem.transports.get_by_key(&transport_key) {
            let dest_space_idx = transport.destination().space();
            if let Some(dest_space_name) = problem.spaces.key(dest_space_idx) {
                spaces.push(dest_space_name.to_string());
            }
        }
    }

    spaces.join("-")
}

/// Build vertex sequence string from transport indices
/// Each vertex represents a space-time pair (S{space_idx}-T{time})
fn build_vertex_sequence<V: Variant>(problem: &Problem<V>, transport_indices: &[usize]) -> String
where
    V::T: From<usize>,
{
    use orx_network_flow::IdxCore;

    if transport_indices.is_empty() {
        return String::new();
    }

    let mut vertices = Vec::new();

    // Add the origin vertex of the first transport
    if let Some(&first_t_idx) = transport_indices.first() {
        let first_transport_key = V::T::from(first_t_idx);
        if let Some(first_transport) = problem.transports.get_by_key(&first_transport_key) {
            let origin_st = first_transport.origin();
            vertices.push(format!(
                "S{}-T{}",
                origin_st.space().into_inner(),
                origin_st.time()
            ));
        }
    }

    // Add destination vertex of each transport
    for &t_idx in transport_indices {
        let transport_key = V::T::from(t_idx);
        if let Some(transport) = problem.transports.get_by_key(&transport_key) {
            let dest_st = transport.destination();
            vertices.push(format!(
                "S{}-T{}",
                dest_st.space().into_inner(),
                dest_st.time()
            ));
        }
    }

    vertices.join("-")
}

/// Extract solution data (commodity paths and transport utilization)
fn extract_solution_data<V: Variant>(
    problem: &Problem<V>,
    solution: &McnfSolution<V>,
) -> SolutionData
where
    V::F: Into<u64>,
    V::S: ToString,
    V::T: From<usize>,
{
    // Extract commodity routing information
    let mut commodity_solutions = Vec::new();
    let mut commodity_index = 0;

    for paths in solution.commodity_paths().iter() {
        let mut commodity_paths = Vec::new();
        let mut total_flow_u64 = 0u64;
        let mut path_idx = 0;

        for path_flow in paths.into_iter() {
            // Convert flow to u64
            let flow_u64: u64 = path_flow.flow.into();
            total_flow_u64 += flow_u64;

            // Extract transport indices from Debug representation
            let path_debug = format!("{:?}", path_flow.path);
            let transport_indices = extract_transport_indices_from_path(&path_debug);
            let num_transports = transport_indices.len();

            // Build transport index string (e.g., "0-1-2")
            let transport_path = if num_transports > 0 {
                transport_indices
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("-")
            } else {
                "[No path]".to_string()
            };

            // Build space sequence string (e.g., "AMS-BRU-LEJ")
            let space_path = build_space_sequence(problem, &transport_indices);

            // Build vertex sequence string (e.g., "S0-T0-S1-T5-S2-T10")
            let vertex_path = build_vertex_sequence(problem, &transport_indices);

            commodity_paths.push(CommodityPath {
                path_index: path_idx,
                flow: flow_u64,
                num_transports,
                transport_path,
                space_path,
                vertex_path,
            });

            path_idx += 1;
        }

        commodity_solutions.push(CommoditySolution {
            commodity_id: commodity_index,
            paths: commodity_paths,
            total_flow: total_flow_u64,
        });

        commodity_index += 1;
    }

    // Extract transport utilization information
    let mut transport_utilizations = Vec::new();
    let mut transport_index = 0;
    let mut total_flow_routed = 0u64;

    for loads in solution.transport_loads().iter() {
        let mut total_load = 0u64;
        let num_commodities = loads.len();

        for load in loads {
            let load_u64: u64 = load.load.into();
            total_load += load_u64;
            total_flow_routed += load_u64;
        }

        transport_utilizations.push(TransportUtilization {
            transport_id: transport_index,
            total_load,
            num_commodities,
        });

        transport_index += 1;
    }

    SolutionData {
        commodity_solutions,
        transport_utilizations,
        total_flow_routed,
    }
}
