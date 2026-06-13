use crate::{Problem, Space, SpaceTime, Time, Variant, vehicles::Vehicle};
use alloc::{format, string::String};

pub struct TransportData<V: Variant> {
    vehicle: Vehicle,
    ori: SpaceTime,
    des: SpaceTime,
    cap: V::F,
}

impl<V: Variant> TransportData<V> {
    pub fn new(vehicle: Vehicle, ori: SpaceTime, des: SpaceTime, cap: V::F) -> Self {
        Self {
            vehicle,
            ori,
            des,
            cap,
        }
    }

    pub fn origin(&self) -> SpaceTime {
        self.ori
    }

    pub fn destination(&self) -> SpaceTime {
        self.des
    }

    pub fn vehicle(&self) -> Vehicle {
        self.vehicle
    }

    pub fn capacity(&self) -> V::F {
        self.cap
    }

    pub fn ori_des(&self) -> [Space; 2] {
        [self.ori.space(), self.des.space()]
    }

    pub fn dt_at(&self) -> [Time; 2] {
        [self.ori.time(), self.des.time()]
    }

    pub fn duration(&self) -> Time {
        self.des.time() - self.ori.time()
    }

    pub(crate) fn var_str(&self, p: &Problem<V>) -> String {
        let [ori, des] = [self.ori, self.des].map(|x| p.space_key(x.space()));
        let [rt, due] = [self.ori.time(), self.des.time()];
        format!("{}_{}_{}_{}", ori, des, rt, due)
    }
}
