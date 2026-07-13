use crate::{Location, problem::space_connectivity::LocationConnectivity};
use core::marker::PhantomData;

#[derive(derive_new::new)]
pub struct AllowAll<L: Location>(PhantomData<L>);

impl<L: Location> LocationConnectivity for AllowAll<L> {
    type L = L;

    fn can_connect(&self, _: Self::L, _: Self::L, _: Self::L) -> bool {
        true
    }
}
