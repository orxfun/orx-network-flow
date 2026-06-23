use good_lp::LpSolver;
use lp_solvers::solvers::Cplex;
use orx_network_flow::graphs::Graph;
use orx_network_flow::graphs::visualization::dot::DotGraph;
use orx_network_flow::{
    McnfSolver, ProblemBuilder, SpaceTimeNwSettings, SpaceTimeRoMcnfParams, Variant,
};

#[derive(Clone, Copy, Default)]
struct MyVariant;

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

fn main() {
    let problem = sample_problem();

    let conn_wait_nw = problem.construct_wait_nw(orx_network_flow::networks::ConnWaitNwSettings {
        add_bypass_edges: true,
    });

    let space_time_nw = problem.construct_space_time_nw(SpaceTimeNwSettings {
        add_bypass_edges: true,
    });

    let dot = space_time_nw.as_dot_graph(None);
    dot.create_svg_file("target/space_time_nw.dot", "target/space_time_nw.svg")
        .unwrap();

    report_complexity(&problem, &conn_wait_nw, &space_time_nw, true);

    let conn_wait_solver =
        McnfSolver::edge_wait_ro(&conn_wait_nw, Default::default(), cplex_solver());
    let conn_wait_sol = conn_wait_solver.solve().expect("conn_wait solution");

    let space_time_solver = McnfSolver::space_time_ro(
        &space_time_nw,
        SpaceTimeRoMcnfParams::default(),
        cplex_solver(),
    );
    space_time_solver.display_lp();
    space_time_solver
        .export_lp("target/space_time_nw.lp")
        .expect("lp");
    let space_time_sol = space_time_solver.solve().expect("space_time solution");

    let dot = dot.with_solution(&space_time_sol);
    dot.create_svg_file("target/space_time_nw.dot", "target/space_time_nw.svg")
        .unwrap();

    println!("=== Commodity Transported Flow (ConnWait vs SpaceTime) ===");
    for (c, com_paths_a) in conn_wait_sol.commodity_paths().enumerated_iter() {
        let com_paths_b = &space_time_sol.commodity_paths()[c];

        let sum_a: u64 = com_paths_a.path_flows.iter().map(|x| x.flow).sum();
        let sum_b: u64 = com_paths_b.path_flows.iter().map(|x| x.flow).sum();

        let key = problem.commodity_key(c);
        println!("commodity {key}: {sum_a} vs {sum_b}");
    }

    println!("\n=== Transport Loads (ConnWait vs SpaceTime) ===");
    for (t, loads_a) in conn_wait_sol.transport_loads().enumerated_iter() {
        let loads_b = &space_time_sol.transport_loads()[t];
        let load_a: u64 = loads_a.iter().map(|x| x.load).sum();
        let load_b: u64 = loads_b.iter().map(|x| x.load).sum();

        println!("transport {t}: {load_a} vs {load_b}");
    }

    println!("\n=== Space-Time Solution Paths ===");
    for (c, paths) in space_time_sol.commodity_paths().enumerated_iter() {
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

fn sample_problem() -> orx_network_flow::Problem<MyVariant> {
    let builder: ProblemBuilder<MyVariant, _> = ProblemBuilder::new();

    let mut builder = builder.with_geographic_spaces([
        ("AMS".to_string(), 52.308_613, 4.763_889),
        ("BRU".to_string(), 50.901_389, 4.484_444),
        ("LEJ".to_string(), 51.25, 12.14),
        ("CVG".to_string(), 39.0488, -84.6678),
        ("SIN".to_string(), 1.350_189, 103.994_433),
        ("EMA".to_string(), 52.831_111, -1.328_056),
    ]);

    // commodities
    let mut c_idx = 0;
    let c = &mut c_idx;
    let mut commodity = |ori: &str, des: &str, rt: i64, due: i64| {
        builder.push_commodity(*c, ori.to_string(), rt, des.to_string(), due, 100);
        *c += 1;
    };

    commodity("AMS", "BRU", 0, 20);
    // commodity("AMS", "CVG", 0, 20);
    // commodity("AMS", "LEJ", 0, 20);
    // commodity("AMS", "LEJ", 0, 20);
    commodity("LEJ", "CVG", 0, 20);

    // transports
    let mut t_idx = 0;
    let t = &mut t_idx;
    let mut transport = |ori: &str, des: &str, dt: i64, at: i64, cap: u64| {
        builder.push_transport(
            *t,
            12,
            String::from("77X"),
            ori.to_string(),
            dt,
            des.to_string(),
            at,
            cap,
        );
        *t += 1;
    };

    transport("AMS", "BRU", 1, 2, 10);
    transport("AMS", "BRU", 4, 5, 10);
    transport("AMS", "LEJ", 4, 5, 10);
    // transport("LEJ", "BRU", 1, 2, 10);
    transport("LEJ", "BRU", 4, 5, 10);
    transport("BRU", "CVG", 7, 12, 10);

    let mut lost_revenue_cost = builder.lost_revenue_cost();
    lost_revenue_cost.commodity_specific(&0, 1);
    lost_revenue_cost.commodity_specific(&1, 3);
    // lost_revenue_cost.commodity_specific(&2, 10);
    // lost_revenue_cost.commodity_specific(&3, 2);
    // lost_revenue_cost.commodity_specific(&4, 8);

    builder.finish()
}

pub fn cplex_solver() -> LpSolver<Cplex> {
    good_lp::LpSolver(Cplex::with_command(
        "/usr/local/cplex/bin/x86-64_linux/cplex".to_string(),
    ))
}

fn report_complexity(
    problem: &orx_network_flow::Problem<MyVariant>,
    conn_wait_nw: &orx_network_flow::networks::ConnWaitNw<'_, MyVariant>,
    space_time_nw: &orx_network_flow::networks::SpaceTimeNw<'_, MyVariant>,
    add_bypass_edges: bool,
) {
    let ro_groups = problem.sorted_ro_commodities.len();
    let transports = problem.len_transports();
    let bypass_edges = match add_bypass_edges {
        true => problem.len_commodities(),
        false => 0,
    };

    let cw = conn_wait_nw.as_dot_graph(None);
    let st = space_time_nw.as_dot_graph(None);

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

    println!("\nConnWait network size:");
    println!("* vertices: {cw_v}");
    println!("* edges:    {cw_e}");

    println!("\nSpaceTime network size:");
    println!("* vertices: {st_v}");
    println!("* edges:    {st_e}");

    println!("\nEstimated RO LP size (ConnWait):");
    println!("* variables:   {cw_vars}");
    println!("* constraints: {cw_cons}");

    println!("\nEstimated RO LP size (SpaceTime):");
    println!("* variables:   {st_vars}");
    println!("* constraints: {st_cons}");
}
