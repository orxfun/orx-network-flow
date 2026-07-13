use crate::Location;

pub trait LocationConnectivity: Default {
    type L: Location;

    fn allow_all(&mut self);

    fn can_connect(&self, a: Self::L, b: Self::L, c: Self::L) -> bool;
}
