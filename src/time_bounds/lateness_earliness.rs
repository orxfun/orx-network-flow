use crate::{commodities::Commodity, std_utils::Map, time::Time};

pub struct LatenessEarlinessBounds {
    global: Time,
    by_commodity: Map<Commodity, Time>,
}

impl LatenessEarlinessBounds {
    pub fn new(global: Time) -> Self {
        Self {
            global,
            by_commodity: Default::default(),
        }
    }

    pub fn global(&mut self, global_bound: Time) {
        self.global = global_bound
    }

    pub fn commodity_specific(&mut self, commodity: Commodity, lateness_bound: Time) {
        self.by_commodity.insert(commodity, lateness_bound);
    }

    pub fn bound(&self, commodity: Commodity) -> Time {
        match self.by_commodity.get(&commodity) {
            Some(bound) => *bound,
            None => self.global,
        }
    }
}
