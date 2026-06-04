use crate::{Problem, Variant, commodities::Commodity, spaces::Space, std_utils::Map, time::Time};

pub struct DepartureBounds {
    global: Time,
    by_space: Map<Space, Time>,
    by_commodity: Map<Commodity, Time>,
}

impl DepartureBounds {
    pub fn new(global: Time) -> Self {
        Self {
            global,
            by_space: Default::default(),
            by_commodity: Default::default(),
        }
    }

    pub fn global(&mut self, global_bound: Time) {
        self.global = global_bound
    }

    pub fn space_specific(&mut self, space: Space, lateness_bound: Time) {
        self.by_space.insert(space, lateness_bound);
    }

    pub fn commodity_specific(&mut self, commodity: Commodity, lateness_bound: Time) {
        self.by_commodity.insert(commodity, lateness_bound);
    }

    pub fn bound<V: Variant>(&self, prob: &Problem<V>, commodity: Commodity) -> Time {
        match self.by_commodity.get(&commodity) {
            Some(bound) => *bound,
            None => {
                let space = prob.commodity_by_idx(commodity).origin().space();
                match self.by_space.get(&space) {
                    Some(bound) => *bound,
                    None => self.global,
                }
            }
        }
    }
}
