use crate::{Variant, commodities::Commodity, cost::Cost, std_utils::Map};

pub struct LostRevenue<V: Variant> {
    global: V::C,
    lost_revenues: Map<Commodity, V::C>,
}

impl<V: Variant> Default for LostRevenue<V> {
    fn default() -> Self {
        Self::new(Cost::zero())
    }
}

impl<V: Variant> LostRevenue<V> {
    pub fn new(global_revenue_per_unit: V::C) -> Self {
        Self {
            global: -global_revenue_per_unit,
            lost_revenues: Default::default(),
        }
    }

    pub fn commodity_specific(&mut self, commodity: Commodity, unit_revenue: V::C) {
        self.lost_revenues.insert(commodity, -unit_revenue);
    }

    pub fn cost(&self, commodity: Commodity) -> V::C {
        match self.lost_revenues.get(&commodity) {
            Some(cost) => *cost,
            None => self.global,
        }
    }
}
