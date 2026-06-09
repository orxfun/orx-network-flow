use crate::Variant;
use crate::commodities::{Commodity, CommodityData};
use crate::{indices::IdxMapSubset, space_time::SpaceTimeOd, std_utils::Map};

pub struct CommoditiesByOdSt<'a, V: Variant> {
    od_commodities: Map<SpaceTimeOd, IdxMapSubset<'a, V::K, CommodityData<V>, Commodity>>,
}
