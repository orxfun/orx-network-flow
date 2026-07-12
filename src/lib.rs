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

mod algorithm;
mod commodities;
mod common_ds;
mod cost;
mod costs;
mod flow_units;
pub mod graphs;
mod indices;
mod mcnf;
pub mod networks;
mod problem;
mod solution_deprecated;
pub mod solvers;
mod spaces;
mod time;
mod time_bounds;
mod transports;
mod utils;
mod vehicle_types;
mod vehicles;

pub use indices::{Idx, IdxCore};
pub use problem::{
    DefiningProblem, DefiningSpaces, EuclideanConnectivity, GeographicalConnectivity, Problem,
    ProblemBuilder, Variant,
};
pub use solution_deprecated::SolutionDeprecated;

pub(crate) use commodities::{Commodities, Commodity, CommodityData};
pub use flow_units::FlowUnit;
pub(crate) use indices::{IdxMap, IdxMapSubset};
pub use mcnf::AoaWaitDdMcnfParams;
pub use mcnf::AoaWaitRoMcnfParams;
pub use mcnf::AonWaitDdMcnfParams;
pub use mcnf::AonWaitRoMcnfParams;
pub use mcnf::{McnfSolution, McnfSolver};
pub use networks::AoaWaitNwSettings;
pub use networks::GraphStats;
pub(crate) use spaces::{Space, SpaceTime, SpaceTimeOd};
pub(crate) use time::Time;
pub(crate) use transports::{Transport, TransportData, Transports, VecTransport};
pub(crate) use vehicles::Vehicle;
