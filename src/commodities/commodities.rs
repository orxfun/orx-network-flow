use crate::commodities::{Commodity, CommodityData};
use crate::space_time::SpaceTime;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Commodities(Vec<CommodityData>);

impl Commodities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, ori: SpaceTime, des: SpaceTime) -> Commodity {
        let commodity = self.0.len().into();
        self.0.push(CommodityData::new(ori, des));
        commodity
    }
}
