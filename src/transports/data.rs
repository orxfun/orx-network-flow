use crate::space_time::SpaceTime;

pub struct TransportData {
    ori: SpaceTime,
    des: SpaceTime,
}

impl TransportData {
    pub fn new(ori: SpaceTime, des: SpaceTime) -> Self {
        Self { ori, des }
    }
}
