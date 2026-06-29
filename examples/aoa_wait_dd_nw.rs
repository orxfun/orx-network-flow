#[path = "shared/shared_problem.rs"]
mod shared_problem;

use orx_network_flow::graphs::visualization::dot::DotGraph;
use orx_network_flow::{AoaWaitDdMcnfParams, AoaWaitNwSettings, McnfSolver};
use shared_problem::{cplex_solver, sample_problem};

fn main() {
    let problem = sample_problem();

    let settings = AoaWaitNwSettings {
        add_bypass_edges: true,
    };
    let nw = problem.construct_aoa_wait_nw(settings);

    let dot = nw.as_dot_graph(None);
    dot.create_svg_file("target/aoa_wait_nw.dot", "target/aoa_wait_nw.svg")
        .unwrap();

    let solver = McnfSolver::aoa_wait_dd(&nw, AoaWaitDdMcnfParams::default(), cplex_solver());
    let stats = solver.stats();
    solver.display_lp();
    solver.export_lp("target/aoa_wait_dd_nw.lp").expect("lp");
    let solution = solver.solve().unwrap();

    let dot = dot.with_solution(&solution).with_stats(stats);
    dot.create_svg_file("target/aoa_wait_dd_nw.dot", "target/aoa_wait_dd_nw.svg")
        .unwrap();

    for (c, paths) in solution.commodity_paths().enumerated_iter() {
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
