use crate::utils::std_utils::Map;
use crate::spaces::Spaces;
use crate::time_bounds::TimeBounds;
use crate::{Commodities, Problem, Variant, commodities::Commodity, spaces::Space, time::Time};

pub struct DepartureBounds {
    global: Time,
    by_space: Map<Space, Time>,
    by_commodity: Map<Commodity, Time>,
}

impl DepartureBounds {
    pub fn new() -> Self {
        Self {
            global: Time::inf(),
            by_space: Default::default(),
            by_commodity: Default::default(),
        }
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

pub struct DepartureTimeBoundsBuilder<'a, V: Variant> {
    spaces: &'a Spaces<V>,
    commodities: &'a Commodities<V>,
    time_bounds: &'a mut TimeBounds,
}

impl<'a, V: Variant> DepartureTimeBoundsBuilder<'a, V> {
    pub(crate) fn new(
        spaces: &'a Spaces<V>,
        commodities: &'a Commodities<V>,
        time_bounds: &'a mut TimeBounds,
    ) -> Self {
        Self {
            spaces,
            commodities,
            time_bounds,
        }
    }

    fn bounds(&mut self) -> &mut DepartureBounds {
        &mut self.time_bounds.max_waiting
    }

    pub fn global(mut self, bound: impl Into<Time>) -> Self {
        self.bounds().global = bound.into();
        self
    }

    pub fn by_space(mut self, space: &V::S, bound: impl Into<Time>) -> Self {
        let space = self
            .spaces
            .get_ind_by_key(space)
            .expect("Space '{space}' does not belong to the problem");
        self.bounds().by_space.insert(space, bound.into());
        self
    }

    pub fn by_commodity(mut self, commodity: &V::K, bound: impl Into<Time>) -> Self {
        let commodity = self
            .commodities
            .get_ind_by_key(commodity)
            .expect("Commodity '{commodity}' does not belong to the problem");
        self.bounds().by_commodity.insert(commodity, bound.into());
        self
    }
}
