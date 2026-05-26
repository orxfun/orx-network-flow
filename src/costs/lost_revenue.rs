use crate::{commodities::Commodity, std_utils::Map};

pub struct LostRevenue {
    lost_revenues: Map<Commodity, u32>,
}
