mod builder;
mod builder_zzz;
mod connectivity;
mod debug;
mod prob;
mod variant;

pub use builder_zzz::{DefiningProblem, DefiningSpaces, ProblemBuilder};
pub use connectivity::{EuclideanConnectivity, GeographicalConnectivity};
pub use prob::Problem;
pub use variant::Variant;
