use crate::{Commodities, Variant, commodities::Commodity, cost::Cost, utils::std_utils::Map};

pub struct LostRevenue<V: Variant> {
    global: V::C,
    by_commodity: Map<Commodity, V::C>,
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
            by_commodity: Default::default(),
        }
    }

    pub fn cost(&self, commodity: Commodity) -> V::C {
        match self.by_commodity.get(&commodity) {
            Some(cost) => *cost,
            None => self.global,
        }
    }
}

#[derive(derive_new::new)]
pub struct LostRevenueBuilder<'a, V: Variant> {
    commodities: &'a Commodities<V>,
    cost: &'a mut LostRevenue<V>,
}

impl<'a, V: Variant> LostRevenueBuilder<'a, V> {
    pub fn global(&mut self, global_revenue_per_unit: V::C) {
        self.cost.global = global_revenue_per_unit
    }

    pub fn commodity_specific(&mut self, commodity: &V::K, unit_revenue: V::C) {
        let commodity = self
            .commodities
            .get_ind_by_key(commodity)
            .expect("Unknown commodity");
        self.cost.by_commodity.insert(commodity, unit_revenue);
    }
}
