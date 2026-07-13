use crate::{NoLocation, problem::space_connectivity::LocationConnectivity};

#[derive(derive_new::new, Default)]
pub struct ConnectivityNoLocation;

impl LocationConnectivity for ConnectivityNoLocation {
    type L = NoLocation;

    fn allow_all(&mut self) {}

    fn can_connect(&self, _: Self::L, _: Self::L, _: Self::L) -> bool {
        true
    }
}
