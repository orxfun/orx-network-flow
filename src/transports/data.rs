use crate::{Variant, space_time::SpaceTime};

pub struct TransportData<V: Variant> {
    ori: SpaceTime,
    des: SpaceTime,
    cap: V::F,
}

impl<V: Variant> TransportData<V> {
    pub fn new(ori: SpaceTime, des: SpaceTime, cap: V::F) -> Self {
        Self { ori, des, cap }
    }

    pub fn origin(&self) -> SpaceTime {
        self.ori
    }

    pub fn destination(&self) -> SpaceTime {
        self.des
    }

    pub fn capacity(&self) -> V::F {
        self.cap
    }
}
