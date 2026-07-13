use crate::problem::LocationConnectivity;

pub trait Location: Clone + Copy {
    type Connectivity: LocationConnectivity<L = Self>;
}
