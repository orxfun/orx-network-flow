use crate::{Variant, commodities::Commodity, std_utils::Map};

pub struct LostRevenue<V: Variant> {
    lost_revenues: Map<Commodity, V::C>,
}

impl<V: Variant> Default for LostRevenue<V> {
    fn default() -> Self {
        Self {
            lost_revenues: Default::default(),
        }
    }
}

impl<V: Variant> LostRevenue<V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, commodity: Commodity, unit_revenue: V::C) {
        self.lost_revenues.insert(commodity, -unit_revenue);
    }
}
