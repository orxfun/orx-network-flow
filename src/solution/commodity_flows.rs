use crate::{Variant, solution::PathFlow};

pub struct CommodityFlows<V: Variant> {
    path_flows: PathFlow<V>,
}
