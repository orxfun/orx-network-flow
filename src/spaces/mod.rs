#[cfg(test)]
mod tests;

mod collection;
mod data;
mod locations;
mod space;
mod space_time;

pub use collection::Spaces;
pub use data::{Coordinate, Geocode, SpaceData};
pub use locations::{Euclidean, Geographical, Location, NoLocation};
pub use space::{Space, VecSpace};
pub use space_time::{SpaceTime, SpaceTimeOd};
