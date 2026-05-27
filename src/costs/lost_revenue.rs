use crate::{
    Problem, Variant, commodities::Commodity, cost::Cost, flow_units::FlowUnit, std_utils::Map,
};

pub struct LostRevenue<V: Variant> {
    global_per_unit: V::C,
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
            global_per_unit: -global_revenue_per_unit,
            lost_revenues: Default::default(),
        }
    }

    pub fn global(&mut self, global_per_unit: V::C) {
        self.global_per_unit = global_per_unit
    }

    pub fn commodity_specific(&mut self, commodity: Commodity, unit_revenue: V::C) {
        self.lost_revenues.insert(commodity, -unit_revenue);
    }

    pub fn cost(&self, prob: &Problem<V>, commodity: Commodity) -> V::C {
        let unit_cost = match self.lost_revenues.get(&commodity) {
            Some(cost) => *cost,
            None => self.global_per_unit,
        };

        let commodity = prob.commodity_by_idx(commodity);
        let amount = V::chargeable_flow(commodity.amount());

        amount * unit_cost
    }
}
