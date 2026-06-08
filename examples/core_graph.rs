// use orx_network_flow::visualization::dot::DotGraph;
// use orx_network_flow::{GeographicalConnectivity, ProblemBuilder, Variant};
// use std::fs;
// use std::process::Command;

// struct MyVariant;

// impl Variant for MyVariant {
//     type S = String;

//     type K = usize;

//     type W = String;

//     type V = usize;

//     type T = usize;

//     type F = u64;

//     type C = i64;

//     fn chargeable_flow(flow: Self::F) -> Self::C {
//         flow as i64
//     }
// }

// fn main() {
//     let builder: ProblemBuilder<MyVariant, _> = ProblemBuilder::new();

//     // let mut builder = builder.with_basic_spaces([
//     //     "AMS".to_string(),
//     //     "BRU".to_string(),
//     //     "SIN".to_string(),
//     //     "EMA".to_string(),
//     // ]);

//     let mut builder = builder.with_geographic_spaces([
//         ("AMS".to_string(), 52.308_613, 4.763_889),
//         ("BRU".to_string(), 50.901_389, 4.484_444),
//         ("CVG".to_string(), 39.0488, -84.6678),
//         ("SIN".to_string(), 1.350_189, 103.994_433),
//         ("EMA".to_string(), 52.831_111, -1.328_056),
//     ]);

//     // commodities
//     let mut c_idx = 0;
//     let c = &mut c_idx;
//     let mut commodity = |ori: &str, des: &str, rt: i64, due: i64| {
//         builder.push_commodity(*c, ori.to_string(), rt, des.to_string(), due, 100);
//         *c += 1;
//     };

//     commodity("AMS", "BRU", 0, 20);
//     commodity("AMS", "CVG", 0, 20);
//     commodity("CVG", "AMS", 0, 20);
//     commodity("CVG", "BRU", 0, 20);

//     // transports
//     let mut t_idx = 0;
//     let t = &mut t_idx;
//     let mut transport = |ori: &str, des: &str, dt: i64, at: i64| {
//         builder.push_transport(
//             *t,
//             12,
//             String::from("77X"),
//             ori.to_string(),
//             dt,
//             des.to_string(),
//             at,
//             1000,
//         );
//         *t += 1;
//     };

//     transport("AMS", "BRU", 1, 2);
//     transport("AMS", "BRU", 4, 5);
//     transport("AMS", "BRU", 7, 8);

//     transport("BRU", "CVG", 1, 6);
//     transport("BRU", "CVG", 7, 12);
//     transport("BRU", "CVG", 13, 18);

//     transport("CVG", "AMS", 1, 5);
//     transport("CVG", "AMS", 4, 8);
//     transport("CVG", "AMS", 7, 11);
//     transport("CVG", "AMS", 10, 14);

//     // settings

//     let geo_conn = GeographicalConnectivity {
//         near_ac_km: 500.0,
//         far_via_b_km: 900.0,
//         min_detour_ratio: 1.8,
//         min_excess_km: 700.0,
//         epsilon_ac_km: 50.0,
//     };
//     builder
//         .spatial_connectivity()
//         .with_geographical_connectivity(geo_conn)
//         .ban_connection(&"AMS".to_string(), &"BRU".to_string(), &"SIN".to_string());
//     builder.temporal_connectivity().global(2i64, 1000i64);

//     builder.max_waiting().global(1000i64);

//     builder.max_earliness().global(1000i64);
//     builder.max_lateness().global(0i64);

//     let problem = builder.finish();

//     let nw = problem.core_network();

//     let dot = nw.dot(None);

//     let dot_text = dot.to_dot_string();
//     println!("{dot_text}");

//     let dot_path = "target/core_graph.dot";
//     let svg_path = "target/core_graph.svg";

//     if let Err(err) = fs::create_dir_all("target") {
//         eprintln!("failed to create target directory: {err}");
//         return;
//     }

//     if let Err(err) = fs::write(dot_path, &dot_text) {
//         eprintln!("failed to write DOT file to {dot_path}: {err}");
//         return;
//     }

//     Command::new("dot")
//         .args(["-Tsvg", dot_path, "-o", svg_path])
//         .status()
//         .expect("failed to create svg");
// }

fn main() {}
