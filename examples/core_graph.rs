use orx_network_flow::graph::visualization::dot::DotGraph;
use orx_network_flow::{ProblemBuilder, Variant};
use std::fs;
use std::process::Command;

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

    // let mut builder = builder.with_basic_spaces([
    //     "AMS".to_string(),
    //     "BRU".to_string(),
    //     "SIN".to_string(),
    //     "EMA".to_string(),
    // ]);
    let mut builder = builder.with_geographic_spaces([
        ("AMS".to_string(), 52.308_613, 4.763_889),
        ("BRU".to_string(), 50.901_389, 4.484_444),
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

    commodity("AMS", "BRU", 3, 16);
    commodity("BRU", "SIN", 6, 17);
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
    // transport("AMS", "BRU", 8, 10);
    // transport("AMS", "BRU", 14, 16);
    // transport("AMS", "BRU", 15, 17);
    transport("AMS", "BRU", 18, 20);
    transport("BRU", "SIN", 10, 15);
    transport("BRU", "SIN", 15, 20);
    transport("BRU", "EMA", 15, 17);
    // transport("BRU", "SIN", 20, 25);
    // transport("BRU", "SIN", 25, 30);
    // transport("AMS", "EMA", 5, 9);
    transport("AMS", "EMA", 12, 16);

    builder.max_waiting().global(1000i64);

    builder.min_conn_time().global(0i64, 0i64);
    builder.max_conn_time().global(1000i64, 1000i64);

    builder.max_earliness().global(1000i64);
    builder.max_lateness().global(0i64);

    let problem = builder.finish();

    let nw = problem.core_network();

    let dot = nw.dot(None);

    let dot_text = dot.to_dot_string();
    println!("{dot_text}");

    let dot_path = "target/core_graph.dot";
    let svg_path = "target/core_graph.svg";

    if let Err(err) = fs::create_dir_all("target") {
        eprintln!("failed to create target directory: {err}");
        return;
    }

    if let Err(err) = fs::write(dot_path, &dot_text) {
        eprintln!("failed to write DOT file to {dot_path}: {err}");
        return;
    }

    Command::new("dot")
        .args(["-Tsvg", dot_path, "-o", svg_path])
        .status()
        .expect("failed to create svg");
}
