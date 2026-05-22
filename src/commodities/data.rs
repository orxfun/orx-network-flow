use crate::space_time::SpaceTime;

#[derive(Debug)]
pub struct CommodityData {
    ori: SpaceTime,
    des: SpaceTime,
}

impl CommodityData {
    pub fn new(ori: SpaceTime, des: SpaceTime) -> Self {
        Self { ori, des }
    }

    pub fn origin(&self) -> SpaceTime {
        self.ori
    }

    pub fn destination(&self) -> SpaceTime {
        self.des
    }
}
