use orx_network_flow::{ProblemBuilder, Variant};

struct MyCommodity {
    origin: String,
    destination: String,
    ready_time: u64,
    due_time: u64,
    amount: u64,
}

impl MyCommodity {
    fn new(
        origin: String,
        destination: String,
        ready_time: u64,
        due_time: u64,
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
}

fn main() {
    let commodities = vec![
        MyCommodity::new(String::from("AMS"), String::from("BRU"), 7, 10, 150),
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
        String::from("77X"),
        String::from("AMS"),
        6u64,
        String::from("BRU"),
        17u64,
        1000,
    );
    builder.push_transport(
        String::from("BRU-LEJ-26"),
        String::from("77X"),
        String::from("BRU"),
        8u64,
        String::from("EMA"),
        12u64,
        800,
    );
    let problem = builder.finish();

    assert_eq!(problem.len_spaces(), 4);
    assert_eq!(problem.len_commodities(), 2);
    assert_eq!(problem.len_transports(), 2);

    println!("{problem:?}");
}
