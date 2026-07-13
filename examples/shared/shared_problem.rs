use orx_network_flow::*;

#[derive(Clone, Copy, Default)]
pub struct MyVariant;

impl Variant for MyVariant {
    type L = NoLocation;
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

pub fn sample_problem() -> orx_network_flow::Problem<MyVariant> {
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
    commodity("AMS", "CVG", 0, 20);
    commodity("AMS", "LEJ", 0, 20);
    commodity("AMS", "LEJ", 0, 20);
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

    transport("LEJ", "BRU", 1, 2, 10);
    transport("LEJ", "BRU", 4, 5, 10);

    transport("BRU", "CVG", 7, 12, 10);

    let mut lost_revenue_cost = builder.lost_revenue_cost();
    lost_revenue_cost.commodity_specific(&0, 1); // AMS-BRU
    lost_revenue_cost.commodity_specific(&1, 3); // LEJ-CVG
    lost_revenue_cost.commodity_specific(&2, 10); // LEJ-CVG-X
    lost_revenue_cost.commodity_specific(&3, 2); // AMS-CVG
    lost_revenue_cost.commodity_specific(&4, 8); // AMS-CVG

    builder.finish()
}

/// Returns a CPLEX solver via lp-solvers runtime bridge.
/// Available when compiled with `solver-lp-solvers` feature.
#[cfg(feature = "solver-lp-solvers")]
pub fn solver() -> good_lp::LpSolver<lp_solvers::solvers::Cplex> {
    solvers::cplex("/usr/local/cplex/bin/x86-64_linux/cplex")
}

/// Returns a microlp solver (pure Rust, no external dependencies).
/// Available when compiled with `solver-microlp` feature.
#[cfg(feature = "solver-microlp")]
pub fn solver()
-> fn(good_lp::variable::UnsolvedProblem) -> good_lp::solvers::microlp::MicroLpProblem {
    solvers::microlp
}
