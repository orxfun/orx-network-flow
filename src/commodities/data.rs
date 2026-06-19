use crate::{Problem, SpaceTime, Variant};
use alloc::{format, string::String};

#[derive(Debug)]
pub struct CommodityData<V: Variant> {
    ori: SpaceTime,
    des: SpaceTime,
    amount: V::F,
}

impl<V: Variant> CommodityData<V> {
    pub fn new(ori: SpaceTime, des: SpaceTime, amount: V::F) -> Self {
        Self { ori, des, amount }
    }

    pub fn origin(&self) -> SpaceTime {
        self.ori
    }

    pub fn destination(&self) -> SpaceTime {
        self.des
    }

    pub fn amount(&self) -> V::F {
        self.amount
    }

    pub(crate) fn var_str(&self, p: &Problem<V>) -> String {
        let [ori, des] = [self.ori, self.des].map(|x| p.space_key(x.space()));
        let [rt, due] = [self.ori.time(), self.des.time()];
        format!("{}_{}_{}_{}", ori, des, rt, due)
    }

    pub fn to_str(&self, p: &Problem<V>) -> String {
        let [ori, des] = [self.ori, self.des].map(|x| p.space_key(x.space()));
        let [rt, due] = [self.ori.time(), self.des.time()];
        let amount = self.amount();
        format!("ori={ori}, des={des}, ready={rt}, due={due}, amount={amount}")
    }
}
