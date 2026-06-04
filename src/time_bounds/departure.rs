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

pub struct DepartureTimeBoundsBuilder<'a, V: Variant>(&'a mut Problem<V>);

impl<'a, V: Variant> DepartureTimeBoundsBuilder<'a, V> {
    pub(crate) fn new(prob: &'a mut Problem<V>) -> Self {
        Self(prob)
    }

    fn bounds(&mut self) -> &mut DepartureBounds {
        &mut self.0.time_bounds.max_waiting
    }

    pub fn global(mut self, global_bound: impl Into<Time>) -> Self {
        self.bounds().global = global_bound.into();
        self
    }

    pub fn space_specific(mut self, space: &V::S, lateness_bound: impl Into<Time>) -> Self {
        let space = self
            .0
            .space_ind(space)
            .expect("Space '{space}' does not belong to the problem");
        self.bounds().by_space.insert(space, lateness_bound.into());
        self
    }

    pub fn commodity_specific(mut self, commodity: &V::K, lateness_bound: impl Into<Time>) -> Self {
        let commodity = self
            .0
            .commodity_ind(commodity)
            .expect("Commodity '{commodity}' does not belong to the problem");
        self.bounds()
            .by_commodity
            .insert(commodity, lateness_bound.into());
        self
    }
}
