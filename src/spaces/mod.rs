#[cfg(test)]
mod tests;

mod collection;
mod data;
mod space;

pub use collection::Spaces;
pub use data::{Coordinate, Geocode, Location, SpaceData};
pub use space::{Space, VecSpace};
