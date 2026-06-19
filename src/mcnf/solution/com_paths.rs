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

impl Path {
    pub fn nth(&self, n: usize) -> Option<Transport> {
        match (self, n) {
            (Self::OneLeg(t), 0) => Some(*t),
            (Self::TwoLegs([t, _]), 0) => Some(*t),
            (Self::TwoLegs([_, t]), 1) => Some(*t),
            (Self::ThreeLegs([t, _, _]), 0) => Some(*t),
            (Self::ThreeLegs([_, t, _]), 1) => Some(*t),
            (Self::ThreeLegs([_, _, t]), 2) => Some(*t),
            (Self::Long(v), _) => v.get(n).copied(),
            _ => None,
        }
    }
}
