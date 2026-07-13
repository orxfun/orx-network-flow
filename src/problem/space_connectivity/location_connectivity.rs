use crate::Location;

pub trait LocationConnectivity {
    type L: Location;

    fn can_connect(&self, a: Self::L, b: Self::L, c: Self::L) -> bool;
}
