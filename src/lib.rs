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

#[cfg(any(test, feature = "std"))]
extern crate std;

extern crate alloc;

#[cfg(test)]
mod tests;

mod commodities;
mod commodity;
mod commodity_data;
mod indices;
pub mod mcnf;
pub mod networks;
mod problem;
mod problem_builder;
mod space_time;
mod spaces;
mod std_utils;
mod time;
