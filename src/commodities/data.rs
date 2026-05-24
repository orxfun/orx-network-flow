use crate::{Variant, space_time::SpaceTime};

#[derive(Debug)]
pub struct CommodityData<V: Variant> {
    ori: SpaceTime,
    des: SpaceTime,
    amount: V::A,
}

impl<V: Variant> CommodityData<V> {
    pub fn new(ori: SpaceTime, des: SpaceTime, amount: V::A) -> Self {
        Self { ori, des, amount }
    }

    pub fn origin(&self) -> SpaceTime {
        self.ori
    }

    pub fn destination(&self) -> SpaceTime {
        self.des
    }

    pub fn amount(&self) -> V::A {
        self.amount
    }
}
