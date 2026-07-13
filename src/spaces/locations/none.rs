use crate::problem::ConnectivityNoLocation;

use super::Location;

impl Location for NoLocation {
    type Connectivity = ConnectivityNoLocation;
}

#[derive(Clone, Copy)]
pub struct NoLocation;
