mod conn;
mod spatial;

pub use conn::Connectivity;
pub use spatial::{
    EuclideanConnectivity, GeographicalConnectivity, SpatialConnectivity,
    SpatialConnectivityBuilder,
};
