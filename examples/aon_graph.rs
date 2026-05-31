use orx_network_flow::graph::visualization::dot::DotGraph;
use orx_network_flow::graph_builders::activity_on_node::visualization::dot::AonDotGraph;
use orx_network_flow::graph_builders::activity_on_node::{AonGraph, build_aon_graph};
use orx_network_flow::{ProblemBuilder, Variant};

struct MyCommodity {
    origin: String,
    destination: String,
    ready_time: i64,
    due_time: i64,
    amount: u64,
}

impl MyCommodity {
    fn new(
        origin: String,
        destination: String,
        ready_time: i64,
        due_time: i64,
        amount: u64,
    ) -> Self {
        Self {
            origin,
            destination,
            ready_time,
            due_time,
            amount,
        }
    }
}

struct MyVariant;

impl Variant for MyVariant {
    type S = String;

    type K = usize;

    type W = String;

    type V = usize;

    type T = String;

    type F = u64;

    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}

fn main() {
    let commodities = vec![
        MyCommodity::new(String::from("AMS"), String::from("BRU"), 7, 19, 150),
        MyCommodity::new(String::from("AMS"), String::from("LEJ"), 9, 12, 290),
    ];

    let mut builder: ProblemBuilder<MyVariant> = ProblemBuilder::new();

    for (k, commodity) in commodities.iter().enumerate() {
        builder.push_commodity(
            k,
            commodity.origin.clone(),
            commodity.ready_time,
            commodity.destination.clone(),
            commodity.due_time,
            commodity.amount,
        );
    }
    builder.push_transport(
        String::from("AMS-BRU-12"),
        12,
        String::from("77X"),
        String::from("AMS"),
        8i64,
        String::from("BRU"),
        17i64,
        1000,
    );
    builder.push_transport(
        String::from("BRU-LEJ-26"),
        11,
        String::from("77X"),
        String::from("BRU"),
        19i64,
        String::from("EMA"),
        22i64,
        800,
    );
    builder.push_transport(
        String::from("BRU-LEJ-33"),
        11,
        String::from("77X"),
        String::from("BRU"),
        18i64,
        String::from("EMA"),
        20i64,
        800,
    );
    let problem = builder.finish();

    let graph = problem.build_aon_graph();

    let dot = AonDotGraph::new(&problem, &graph);

    println!("{}", dot.to_dot_string());
}
