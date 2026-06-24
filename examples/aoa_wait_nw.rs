#[path = "shared/shared_problem.rs"]
mod shared_problem;

use orx_network_flow::graphs::Graph;
use orx_network_flow::graphs::visualization::dot::DotGraph;
use orx_network_flow::{AoaWaitNwSettings, AoaWaitRoMcnfParams, McnfSolver};
use shared_problem::{MyVariant, cplex_solver, sample_problem};

fn main() {
    let problem = sample_problem();

    let aon_wait_nw =
        problem.construct_aon_wait_nw(orx_network_flow::networks::AonWaitNwSettings {
            add_bypass_edges: true,
        });

    let aoa_wait_nw = problem.construct_aoa_wait_nw(AoaWaitNwSettings {
        add_bypass_edges: true,
    });

    let dot = aoa_wait_nw.as_dot_graph(None);
    dot.create_svg_file("target/aoa_wait_nw.dot", "target/aoa_wait_nw.svg")
        .unwrap();

    report_complexity(&problem, &aon_wait_nw, &aoa_wait_nw, true);

    let aon_wait_solver = McnfSolver::aon_wait_ro(&aon_wait_nw, Default::default(), cplex_solver());
    let aon_wait_sol = aon_wait_solver.solve().expect("aon_wait solution");

    let aoa_wait_solver =
        McnfSolver::aoa_wait_ro(&aoa_wait_nw, AoaWaitRoMcnfParams::default(), cplex_solver());
    aoa_wait_solver.display_lp();
    aoa_wait_solver
        .export_lp("target/aoa_wait_nw.lp")
        .expect("lp");
    let stats = aoa_wait_solver.stats();
    let aoa_wait_sol = aoa_wait_solver.solve().expect("aoa_wait solution");

    let dot = dot.with_solution(&aoa_wait_sol).with_stats(stats);
    dot.create_svg_file("target/aoa_wait_nw.dot", "target/aoa_wait_nw.svg")
        .unwrap();

    println!("=== Commodity Transported Flow (AonWait vs AoaWait) ===");
    for (c, com_paths_a) in aon_wait_sol.commodity_paths().enumerated_iter() {
        let com_paths_b = &aoa_wait_sol.commodity_paths()[c];

        let sum_a: u64 = com_paths_a.path_flows.iter().map(|x| x.flow).sum();
        let sum_b: u64 = com_paths_b.path_flows.iter().map(|x| x.flow).sum();

        let key = problem.commodity_key(c);
        println!("commodity {key}: {sum_a} vs {sum_b}");
    }

    println!("\n=== Transport Loads (AonWait vs AoaWait) ===");
    for (t, loads_a) in aon_wait_sol.transport_loads().enumerated_iter() {
        let loads_b = &aoa_wait_sol.transport_loads()[t];
        let load_a: u64 = loads_a.iter().map(|x| x.load).sum();
        let load_b: u64 = loads_b.iter().map(|x| x.load).sum();

        println!("transport {t}: {load_a} vs {load_b}");
    }

    println!("\n=== AoaWait Solution Paths ===");
    for (c, paths) in aoa_wait_sol.commodity_paths().enumerated_iter() {
        let com = problem.commodity_key(c);
        let commodity = problem.commodity_by_idx(c).to_str(&problem);
        println!("c{com} = {commodity}");
        for path_flow in paths {
            println!(
                "* {}\t{}\t{}",
                path_flow.path,
                path_flow.path.to_str_as_spaces(&problem),
                path_flow.flow
            );
        }
    }
}

fn report_complexity(
    problem: &orx_network_flow::Problem<MyVariant>,
    aon_wait_nw: &orx_network_flow::networks::AonWaitNw<'_, MyVariant>,
    aoa_wait_nw: &orx_network_flow::networks::AoaWaitNw<'_, MyVariant>,
    add_bypass_edges: bool,
) {
    let ro_groups = problem.sorted_ro_commodities.len();
    let transports = problem.len_transports();
    let bypass_edges = match add_bypass_edges {
        true => problem.len_commodities(),
        false => 0,
    };

    let cw = aon_wait_nw.as_dot_graph(None);
    let st = aoa_wait_nw.as_dot_graph(None);

    let cw_v = cw.graph().v();
    let cw_e = cw.graph().e();
    let st_v = st.graph().v();
    let st_e = st.graph().e();

    let estimate_lp = |v: usize, e: usize| {
        // RO model: one variable set per RO for non-bypass edges,
        // one bypass variable per commodity, and one dummy variable.
        let vars = 1 + ro_groups * (e.saturating_sub(bypass_edges)) + bypass_edges;
        // Flow-balance per (RO, vertex) and one capacity per transport.
        let constraints = ro_groups * v + transports;
        (vars, constraints)
    };

    let (cw_vars, cw_cons) = estimate_lp(cw_v, cw_e);
    let (st_vars, st_cons) = estimate_lp(st_v, st_e);

    println!("\n=== Complexity Report ===");
    println!("RO groups (R): {ro_groups}");
    println!("Transports (T): {transports}");
    println!("Bypass edges (B): {bypass_edges}");

    println!("\nAonWait network size:");
    println!("* vertices: {cw_v}");
    println!("* edges:    {cw_e}");

    println!("\nAoaWait network size:");
    println!("* vertices: {st_v}");
    println!("* edges:    {st_e}");

    println!("\nEstimated RO LP size (AonWait):");
    println!("* variables:   {cw_vars}");
    println!("* constraints: {cw_cons}");

    println!("\nEstimated RO LP size (AoaWait):");
    println!("* variables:   {st_vars}");
    println!("* constraints: {st_cons}");
}
