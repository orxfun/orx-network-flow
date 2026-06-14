use orx_network_flow::graphs::visualization::dot::DotGraph;
use orx_network_flow::networks::ConnWaitNwSettings;
use orx_network_flow::{ProblemBuilder, Variant};

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
    commodity("LEJ", "CVG", 0, 20);
    // commodity("LEJ", "CVG", 0, 20);
    // commodity("AMS", "CVG", 0, 20);

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

    transport("LEJ", "BRU", 1, 2, 10);
    transport("LEJ", "BRU", 4, 5, 10);

    transport("BRU", "CVG", 7, 12, 10);
    // transport("BRU", "CVG", 13, 18, 10);

    let mut lost_revenue_cost = builder.lost_revenue_cost();
    lost_revenue_cost.commodity_specific(&0, 1); // AMS-BRU
    lost_revenue_cost.commodity_specific(&1, 3); // LEJ-CVG
    // lost_revenue_cost.commodity_specific(&2, 10); // LEJ-CVG-X
    // lost_revenue_cost.commodity_specific(&3, 2); // AMS-CVG

    let problem = builder.finish();

    let settings = ConnWaitNwSettings {
        add_bypass_edges: true,
    };
    let nw = problem.construct_wait_nw(settings);

    let dot = nw.as_dot_graph(None);
    dot.create_svg_file("target/conn_wait_nw.dot", "target/conn_wait_nw.svg")
        .unwrap();

    let output = nw.solve(true);
    let dot = dot.with_flows(&output.edge_flows);
    dot.create_svg_file("target/conn_wait_nw.dot", "target/conn_wait_nw.svg")
        .unwrap();
}
