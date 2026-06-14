use crate::{Transport, Variant};
use alloc::vec::Vec;

#[derive(Default)]
pub struct CommodityPaths<V: Variant> {
    pub path_flows: Vec<PathFlow<V>>,
}

pub struct PathFlow<V: Variant> {
    pub path: Path,
    pub flow: V::F,
}

pub enum Path {
    OneLeg(Transport),
    TwoLegs([Transport; 2]),
    ThreeLegs([Transport; 3]),
    Long(Vec<Transport>),
}
