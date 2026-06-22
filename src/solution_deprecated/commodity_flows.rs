use crate::{Variant, solution_deprecated::PathFlow};
use alloc::vec::Vec;

#[derive(Default)]
pub struct CommodityFlows<V: Variant> {
    path_flows: Vec<PathFlow<V>>,
}
