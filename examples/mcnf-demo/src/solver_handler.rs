use crate::problem_builder::MyVariant;
use crate::serialization::{
    CommodityAssignment, CommodityDetail, CommodityPath, CommoditySolution, EnhancedSolutionData,
    McnfResponse, NetworkChoice, ProblemInput, SolutionData, TransportDetail, TransportUtilization,
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
            let objective_value = compute_objective_value(&problem, &solution);
            let solution_data = extract_solution_data(&problem, &solution);
            let mut esd = extract_enhanced_solution_data(&problem, &solution);
            esd.commodity_dot = generate_commodity_dot(&esd);
            esd.transport_dot = generate_transport_dot(&esd);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: Some(esd),
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
            let objective_value = compute_objective_value(&problem, &solution);
            let solution_data = extract_solution_data(&problem, &solution);
            let mut esd = extract_enhanced_solution_data(&problem, &solution);
            esd.commodity_dot = generate_commodity_dot(&esd);
            esd.transport_dot = generate_transport_dot(&esd);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: Some(esd),
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
            let objective_value = compute_objective_value(&problem, &solution);
            let solution_data = extract_solution_data(&problem, &solution);
            let mut esd = extract_enhanced_solution_data(&problem, &solution);
            esd.commodity_dot = generate_commodity_dot(&esd);
            esd.transport_dot = generate_transport_dot(&esd);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: Some(esd),
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
            let objective_value = compute_objective_value(&problem, &solution);
            let solution_data = extract_solution_data(&problem, &solution);
            let mut esd = extract_enhanced_solution_data(&problem, &solution);
            esd.commodity_dot = generate_commodity_dot(&esd);
            esd.transport_dot = generate_transport_dot(&esd);

            Ok(McnfResponse {
                num_variables: stats.num_variables,
                num_constraints: stats.num_constraints,
                num_commodities: problem.len_commodities(),
                num_spaces: problem.len_spaces(),
                num_transports: problem.len_transports(),
                objective_value: Some(objective_value),
                status: Some("optimal".to_string()),
                solution_data: Some(solution_data),
                enhanced_solution_data: Some(esd),
            })
        }
        _ => Err("Unreachable: network type and grouping should have been validated".into()),
    }
}

/// Compute objective value from solution by summing flows
fn compute_objective_value<V: Variant>(
    problem: &Problem<V>,
    solution: &McnfSolution<V>,
) -> f64
where
    V::F: Into<u64>,
    V::C: Into<i64>,
{
    use orx_network_flow::IdxCore;

    // Objective = sum over each commodity of (unrouted_flow * lost_revenue_cost_per_unit)
    // Unrouted flow = commodity amount - sum of all routed flows for that commodity
    let mut total_cost: i64 = 0;

    // Sum routed flow per commodity from transport_loads
    let mut routed_per_commodity: std::collections::HashMap<usize, u64> =
        std::collections::HashMap::new();
    for loads in solution.transport_loads().iter() {
        for load in loads {
            let flow: u64 = load.load.into();
            if flow > 0 {
                *routed_per_commodity
                    .entry(load.commodity.into_inner())
                    .or_insert(0) += flow;
            }
        }
    }

    // For each commodity compute unrouted flow × lost revenue cost
    for (c_idx, _key, c_data) in problem.commodities.entries() {
        let quantity: u64 = c_data.amount().into();
        let routed = *routed_per_commodity.get(&c_idx.into_inner()).unwrap_or(&0);
        let unrouted = quantity.saturating_sub(routed);
        if unrouted > 0 {
            // lost_revenue.cost() returns a negative cost (penalty); negate it to get the revenue
            let cost_per_unit: i64 = problem.costs.lost_revenue.cost(c_idx).into();
            total_cost += (unrouted as i64) * (-cost_per_unit);
        }
    }

    total_cost as f64
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

/// Extract enhanced solution data with commodity-centric and transport-centric perspectives
fn extract_enhanced_solution_data<V: Variant>(
    problem: &Problem<V>,
    solution: &McnfSolution<V>,
) -> EnhancedSolutionData
where
    V::F: Into<u64> + Copy,
    V::S: ToString,
    V::T: From<usize>,
{
    use orx_network_flow::IdxCore;

    // ── Commodity-centric view ───────────────────────────────────────────────
    let mut commodity_details = Vec::new();
    let mut commodity_index = 0usize;

    for paths in solution.commodity_paths().iter() {
        let mut commodity_paths = Vec::new();
        let mut total_flow_u64 = 0u64;
        let mut all_transport_ids: Vec<usize> = Vec::new();
        let mut path_idx = 0usize;

        for path_flow in paths.into_iter() {
            let flow_u64: u64 = path_flow.flow.into();
            total_flow_u64 += flow_u64;

            let path_debug = format!("{:?}", path_flow.path);
            let transport_indices = extract_transport_indices_from_path(&path_debug);
            let num_transports = transport_indices.len();

            let transport_path = if num_transports > 0 {
                transport_indices
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("-")
            } else {
                "[No path]".to_string()
            };
            let space_path = build_space_sequence(problem, &transport_indices);
            let vertex_path = build_vertex_sequence(problem, &transport_indices);

            for &t in &transport_indices {
                if !all_transport_ids.contains(&t) {
                    all_transport_ids.push(t);
                }
            }

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

        // Resolve origin/destination space names from the problem
        // Commodity internal index matches iteration order
        let commodity_idx_key = problem
            .commodities
            .entries()
            .nth(commodity_index)
            .map(|(idx, _key, _data)| idx);
        let (origin_space, destination_space) = if let Some(c_idx) = commodity_idx_key {
            if let Some(c_data) = problem.commodities.get_by_idx(c_idx) {
                let ori = problem
                    .spaces
                    .key(c_data.origin().space())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let des = problem
                    .spaces
                    .key(c_data.destination().space())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                (ori, des)
            } else {
                (String::new(), String::new())
            }
        } else {
            (String::new(), String::new())
        };

        all_transport_ids.sort_unstable();
        commodity_details.push(CommodityDetail {
            commodity_id: commodity_index,
            total_flow: total_flow_u64,
            paths: commodity_paths,
            transport_ids: all_transport_ids,
            origin_space,
            destination_space,
        });
        commodity_index += 1;
    }

    // ── Transport-centric view ───────────────────────────────────────────────
    let mut transport_details = Vec::new();
    let mut total_flow_routed = 0u64;
    let mut transport_index = 0usize;

    for loads in solution.transport_loads().iter() {
        let transport_key = V::T::from(transport_index);
        let transport_data = problem.transports.get_by_key(&transport_key);

        let capacity: u64 = transport_data.map(|t| t.capacity().into()).unwrap_or(0u64);

        let (origin_space, destination_space, departure_time, arrival_time) =
            if let Some(t) = transport_data {
                let ori = problem
                    .spaces
                    .key(t.origin().space())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let des = problem
                    .spaces
                    .key(t.destination().space())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let dep: i64 = format!("{}", t.origin().time()).parse().unwrap_or(0);
                let arr: i64 = format!("{}", t.destination().time()).parse().unwrap_or(0);
                (ori, des, dep, arr)
            } else {
                (String::new(), String::new(), 0i64, 0i64)
            };

        let mut per_commodity: std::collections::HashMap<usize, u64> =
            std::collections::HashMap::new();
        let mut utilized_capacity = 0u64;

        for load in loads {
            let flow_u64: u64 = load.load.into();
            if flow_u64 > 0 {
                let c_id = load.commodity.into_inner();
                *per_commodity.entry(c_id).or_insert(0) += flow_u64;
                utilized_capacity += flow_u64;
                total_flow_routed += flow_u64;
            }
        }

        let mut commodity_ids: Vec<usize> = per_commodity.keys().copied().collect();
        commodity_ids.sort_unstable();

        let assigned_commodities = commodity_ids
            .into_iter()
            .map(|c_id| {
                let assigned_flow = per_commodity[&c_id];
                let num_paths = commodity_details
                    .get(c_id)
                    .map(|cd| {
                        let t_str = transport_index.to_string();
                        cd.paths
                            .iter()
                            .filter(|p| {
                                p.transport_path.split('-').any(|seg| seg == t_str.as_str())
                            })
                            .count()
                    })
                    .unwrap_or(0);
                CommodityAssignment {
                    commodity_id: c_id,
                    assigned_flow,
                    num_paths,
                }
            })
            .collect();

        let utilization_rate = if capacity > 0 {
            utilized_capacity as f64 / capacity as f64
        } else {
            0.0
        };

        transport_details.push(TransportDetail {
            transport_id: transport_index,
            capacity,
            utilized_capacity,
            utilization_rate,
            assigned_commodities,
            origin_space,
            destination_space,
            departure_time,
            arrival_time,
        });
        transport_index += 1;
    }

    EnhancedSolutionData {
        total_flow_routed,
        commodity_details,
        transport_details,
        commodity_dot: String::new(), // populated below
        transport_dot: String::new(), // populated below
    }
}

/// Generate a Graphviz dot string for the commodity ↔ transport bipartite graph
pub fn generate_commodity_dot(esd: &EnhancedSolutionData) -> String {
    let mut out = String::from(
        "digraph CommodityNetwork {\n  rankdir=LR;\n  \
         node [fontname=\"Helvetica\",fontsize=11];\n  \
         edge [fontsize=10];\n\n",
    );

    // Commodity nodes (left)
    out.push_str("  { rank=same;\n");
    for cd in &esd.commodity_details {
        let label = format!(
            "C{}\\n{}→{}\\nflow={}",
            cd.commodity_id, cd.origin_space, cd.destination_space, cd.total_flow
        );
        let color = if cd.total_flow > 0 {
            "#d0e8ff"
        } else {
            "#f5f5f5"
        };
        out.push_str(&format!(
            "    C{} [label=\"{}\",shape=box,style=filled,fillcolor=\"{}\"];\n",
            cd.commodity_id, label, color
        ));
    }
    out.push_str("  }\n\n");

    // Transport nodes (right) — only those with activity
    out.push_str("  { rank=same;\n");
    for td in &esd.transport_details {
        if td.assigned_commodities.is_empty() {
            continue;
        }
        let pct = (td.utilization_rate * 100.0).round() as u64;
        let fill = if td.utilization_rate >= 0.8 {
            "#c8e6c9"
        } else if td.utilization_rate >= 0.4 {
            "#fff9c4"
        } else {
            "#ffcdd2"
        };
        let label = format!(
            "T{}\\n{}→{}\\n{}/{}  ({}%)",
            td.transport_id,
            td.origin_space,
            td.destination_space,
            td.utilized_capacity,
            td.capacity,
            pct
        );
        out.push_str(&format!(
            "    T{} [label=\"{}\",shape=ellipse,style=filled,fillcolor=\"{}\"];\n",
            td.transport_id, label, fill
        ));
    }
    out.push_str("  }\n\n");

    // Edges: commodity → transport (labeled with flow)
    for td in &esd.transport_details {
        for ca in &td.assigned_commodities {
            out.push_str(&format!(
                "  C{} -> T{} [label=\"{}\",penwidth={:.1}];\n",
                ca.commodity_id,
                td.transport_id,
                ca.assigned_flow,
                1.0 + ca.assigned_flow as f64 * 0.3
            ));
        }
    }

    out.push_str("}\n");
    out
}

/// Generate a Graphviz dot string for the transport ↔ commodity bipartite graph
pub fn generate_transport_dot(esd: &EnhancedSolutionData) -> String {
    let mut out = String::from(
        "digraph TransportNetwork {\n  rankdir=LR;\n  \
         node [fontname=\"Helvetica\",fontsize=11];\n  \
         edge [fontsize=10];\n\n",
    );

    // Transport nodes (left)
    out.push_str("  { rank=same;\n");
    for td in &esd.transport_details {
        let pct = (td.utilization_rate * 100.0).round() as u64;
        let fill = if td.utilization_rate >= 0.8 {
            "#c8e6c9"
        } else if td.utilization_rate >= 0.4 {
            "#fff9c4"
        } else if td.capacity == 0 {
            "#f5f5f5"
        } else {
            "#ffcdd2"
        };
        let label = format!(
            "T{}\\n{}→{}\\n@{} {}%",
            td.transport_id, td.origin_space, td.destination_space, td.departure_time, pct
        );
        out.push_str(&format!(
            "    T{} [label=\"{}\",shape=ellipse,style=filled,fillcolor=\"{}\"];\n",
            td.transport_id, label, fill
        ));
    }
    out.push_str("  }\n\n");

    // Commodity nodes (right) — only those with flow
    out.push_str("  { rank=same;\n");
    for cd in &esd.commodity_details {
        if cd.total_flow == 0 {
            continue;
        }
        let label = format!(
            "C{}\\n{}→{}\\nflow={}",
            cd.commodity_id, cd.origin_space, cd.destination_space, cd.total_flow
        );
        out.push_str(&format!(
            "    C{} [label=\"{}\",shape=box,style=filled,fillcolor=\"#d0e8ff\"];\n",
            cd.commodity_id, label
        ));
    }
    out.push_str("  }\n\n");

    // Edges: transport → commodity
    for td in &esd.transport_details {
        for ca in &td.assigned_commodities {
            out.push_str(&format!(
                "  T{} -> C{} [label=\"{}\",penwidth={:.1}];\n",
                td.transport_id,
                ca.commodity_id,
                ca.assigned_flow,
                1.0 + ca.assigned_flow as f64 * 0.3
            ));
        }
    }

    out.push_str("}\n");
    out
}
