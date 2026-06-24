#[path = "shared/shared_problem.rs"]
mod shared_problem;

use orx_network_flow::McnfSolver;
use orx_network_flow::graphs::visualization::dot::DotGraph;
use orx_network_flow::networks::AonWaitNwSettings;
use shared_problem::{cplex_solver, sample_problem};

fn main() {
    let problem = sample_problem();

    let settings = AonWaitNwSettings {
        add_bypass_edges: true,
    };
    let nw = problem.construct_aon_wait_nw(settings);

    let dot = nw.as_dot_graph(None);
    dot.create_svg_file("target/aon_wait_nw.dot", "target/aon_wait_nw.svg")
        .unwrap();

    let solver = McnfSolver::aon_wait_ro(&nw, Default::default(), cplex_solver());
    solver.display_lp();
    solver.export_lp("target/aon_wait_nw.lp").expect("lp");
    let solution = solver.solve().unwrap();

    let dot = dot.with_solution(&solution);
    dot.create_svg_file("target/aon_wait_nw.dot", "target/aon_wait_nw.svg")
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
