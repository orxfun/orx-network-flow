use orx_network_flow::{ProblemBuilder, Variant};
use std::marker::PhantomData;

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

struct MyVariant<'a>(PhantomData<&'a ()>);

impl<'a> Variant for MyVariant<'a> {
    type S = &'a str;

    type K = usize;

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
            &commodity.origin,
            commodity.ready_time,
            &commodity.destination,
            commodity.due_time,
            commodity.amount,
        );
    }
    let problem = builder.finish();

    assert_eq!(problem.len_commodities(), 2);
    assert_eq!(problem.len_spaces(), 3);

    println!("{problem:?}");
}
