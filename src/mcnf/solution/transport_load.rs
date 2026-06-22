use crate::{Commodity, Variant};

pub struct CommodityLoad<V: Variant> {
    pub commodity: Commodity,
    pub load: V::F,
}
