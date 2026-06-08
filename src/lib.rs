#![doc = include_str!("../README.md")]
#![warn(
    // missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![no_std]

extern crate alloc;
#[cfg(any(test, feature = "std"))]
extern crate std;

mod commodities;
mod cost;
mod costs;
mod flow_units;
mod graph;
mod graph_extended;
mod indices;
mod networks;
mod problem;
mod space_time;
mod spaces;
mod std_utils;
mod time;
mod time_bounds;
mod transports;
mod vehicle_types;
mod vehicles;

pub use graph::visualization;
pub use graph::{EIdx, Graph, VIdx};
pub use networks::CoreNw;
pub use problem::{
    EuclideanConnectivity, GeographicalConnectivity, Problem, ProblemBuilder, Variant,
};
