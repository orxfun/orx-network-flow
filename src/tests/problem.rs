use crate::problem::ProblemBuilder;
use alloc::string::String;
use alloc::vec;

struct MyCommodity {
    origin: String,
    destination: String,
    ready_time: u64,
    due_time: u64,
}

impl MyCommodity {
    fn new(origin: String, destination: String, ready_time: u64, due_time: u64) -> Self {
        Self {
            origin,
            destination,
            ready_time,
            due_time,
        }
    }
}

#[test]
fn build_problem_push() {
    let commodities = vec![
        MyCommodity::new(String::from("AMS"), String::from("BRU"), 7, 10),
        MyCommodity::new(String::from("AMS"), String::from("LEJ"), 9, 12),
    ];

    let mut builder: ProblemBuilder<&str, usize> = ProblemBuilder::new();

    for (k, commodity) in commodities.iter().enumerate() {
        builder.push_commodity(
            k,
            &commodity.origin,
            commodity.ready_time,
            &commodity.destination,
            commodity.due_time,
        );
    }
    let problem = builder.finish();

    assert_eq!(problem.len_commodities(), 2);
    assert_eq!(problem.len_spaces(), 3);
}
