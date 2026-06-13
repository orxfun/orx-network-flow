use crate::{Variant, solution::PathFlow};
use alloc::vec::Vec;

#[derive(Default)]
pub struct CommodityFlows<V: Variant> {
    path_flows: Vec<PathFlow<V>>,
}
