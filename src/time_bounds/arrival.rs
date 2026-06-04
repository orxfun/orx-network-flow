use crate::{Problem, Variant, commodities::Commodity, spaces::Space, std_utils::Map, time::Time};

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
    p: &'a mut Problem<V>,
    bound_type: ArrivalBoundType,
}

impl<'a, V: Variant> ArrivalTimeBoundsBuilder<'a, V> {
    fn new(p: &'a mut Problem<V>, bound_type: ArrivalBoundType) -> Self {
        Self { p, bound_type }
    }

    pub(crate) fn earliness(p: &'a mut Problem<V>) -> Self {
        Self::new(p, ArrivalBoundType::Earliness)
    }

    pub(crate) fn lateness(p: &'a mut Problem<V>) -> Self {
        Self::new(p, ArrivalBoundType::Lateness)
    }

    fn bounds(&mut self) -> &mut ArrivalBounds {
        match self.bound_type {
            ArrivalBoundType::Earliness => &mut self.p.time_bounds.max_earliness,
            ArrivalBoundType::Lateness => &mut self.p.time_bounds.max_lateness,
        }
    }

    pub fn global(mut self, bound: impl Into<Time>) -> Self {
        self.bounds().global = bound.into();
        self
    }

    pub fn by_space(mut self, space: &V::S, bound: impl Into<Time>) -> Self {
        let space = self
            .p
            .space_ind(space)
            .expect("Space '{space}' does not belong to the problem");
        self.bounds().by_space.insert(space, bound.into());
        self
    }

    pub fn by_commodity(mut self, commodity: &V::K, bound: impl Into<Time>) -> Self {
        let commodity = self
            .p
            .commodity_ind(commodity)
            .expect("Commodity '{commodity}' does not belong to the problem");
        self.bounds().by_commodity.insert(commodity, bound.into());
        self
    }
}
