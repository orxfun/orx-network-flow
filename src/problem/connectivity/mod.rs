mod conn;
mod spatial;
mod temporal;

pub use conn::Connectivity;
pub use spatial::{
    EuclideanConnectivity, GeographicalConnectivity, SpatialConnectivity,
    SpatialConnectivityBuilder,
};
pub use temporal::{TemporalConnectivity, TemporalConnectivityBuilder};
