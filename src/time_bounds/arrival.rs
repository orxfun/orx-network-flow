use crate::spaces::Spaces;
use crate::time_bounds::TimeBounds;
use crate::utils::std_utils::Map;
use crate::{Commodities, Problem, Variant, commodities::Commodity, spaces::Space, time::Time};

pub struct ArrivalBounds {
    global: Time,
    by_space: Map<Space, Time>,
    by_commodity: Map<Commodity, Time>,
}

impl ArrivalBounds {
    pub fn new_earliness() -> Self {
        Self {
            global: Time::inf(),
            by_space: Default::default(),
            by_commodity: Default::default(),
        }
    }

    pub fn new_lateness() -> Self {
        Self {
            global: Time::zero(),
            by_space: Default::default(),
            by_commodity: Default::default(),
        }
    }

    pub fn bound<V: Variant>(&self, prob: &Problem<V>, commodity: Commodity) -> Time {
        match self.by_commodity.get(&commodity) {
            Some(bound) => *bound,
            None => {
                let space = prob.commodity_by_idx(commodity).destination().space();
                match self.by_space.get(&space) {
                    Some(bound) => *bound,
                    None => self.global,
                }
            }
        }
    }
}

pub enum ArrivalBoundType {
    Earliness,
    Lateness,
}

pub struct ArrivalTimeBoundsBuilder<'a, V: Variant> {
    spaces: &'a Spaces<V>,
    commodities: &'a Commodities<V>,
    time_bounds: &'a mut TimeBounds,
    bound_type: ArrivalBoundType,
}

impl<'a, V: Variant> ArrivalTimeBoundsBuilder<'a, V> {
    fn new(
        spaces: &'a Spaces<V>,
        commodities: &'a Commodities<V>,
        time_bounds: &'a mut TimeBounds,
        bound_type: ArrivalBoundType,
    ) -> Self {
        Self {
            spaces,
            commodities,
            time_bounds,
            bound_type,
        }
    }

    pub(crate) fn earliness(
        spaces: &'a Spaces<V>,
        commodities: &'a Commodities<V>,
        time_bounds: &'a mut TimeBounds,
    ) -> Self {
        Self::new(
            spaces,
            commodities,
            time_bounds,
            ArrivalBoundType::Earliness,
        )
    }

    pub(crate) fn lateness(
        spaces: &'a Spaces<V>,
        commodities: &'a Commodities<V>,
        time_bounds: &'a mut TimeBounds,
    ) -> Self {
        Self::new(spaces, commodities, time_bounds, ArrivalBoundType::Lateness)
    }

    fn bounds(&mut self) -> &mut ArrivalBounds {
        match self.bound_type {
            ArrivalBoundType::Earliness => &mut self.time_bounds.max_earliness,
            ArrivalBoundType::Lateness => &mut self.time_bounds.max_lateness,
        }
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
