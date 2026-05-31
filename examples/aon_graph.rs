use orx_network_flow::graph::visualization::dot::DotGraph;
use orx_network_flow::graph_builders::activity_on_node::visualization::dot::AonDotGraph;
use orx_network_flow::{ProblemBuilder, Variant};

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
    let mut builder: ProblemBuilder<MyVariant> = ProblemBuilder::new();

    // commodities
    let mut c_idx = 0;
    let c = &mut c_idx;
    let mut commodity = |ori: &str, des: &str, rt: i64, due: i64| {
        builder.push_commodity(*c, ori.to_string(), rt, des.to_string(), due, 100);
        *c += 1;
    };

    commodity("AMS", "BRU", 5, 16);
    commodity("AMS", "SIN", 9, 26);

    // transports
    let mut t_idx = 0;
    let t = &mut t_idx;
    let mut transport = |ori: &str, des: &str, dt: i64, at: i64| {
        builder.push_transport(
            *t,
            12,
            String::from("77X"),
            ori.to_string(),
            dt,
            des.to_string(),
            at,
            1000,
        );
        *t += 1;
    };

    transport("AMS", "BRU", 4, 6);
    transport("AMS", "BRU", 8, 10);
    transport("AMS", "BRU", 14, 16);
    transport("AMS", "BRU", 15, 17);
    transport("AMS", "BRU", 18, 20);
    transport("BRU", "SIN", 10, 15);
    transport("BRU", "SIN", 15, 20);
    transport("BRU", "SIN", 20, 25);
    transport("BRU", "SIN", 25, 30);

    let problem = builder.finish();

    let graph = problem.build_aon_graph();

    let dot = AonDotGraph::new(&problem, &graph);

    println!("{}", dot.to_dot_string());
}
