use crate::commodities::Commodities;
use crate::commodities::CommodityData;
use crate::problem::variant::Variant;
use crate::spaces::Spaces;
use crate::transports::TransportData;
use crate::transports::Transports;
use crate::vehicle_types::VehicleTypes;

pub struct Problem<V: Variant> {
    pub(super) spaces: Spaces<V::S>,
    pub(super) vehicle_types: VehicleTypes<V>,
    pub(super) commodities: Commodities<V>,
    pub(super) transports: Transports<V>,
}

impl<V: Variant> Problem<V> {
    pub fn len_spaces(&self) -> usize {
        self.spaces.len()
    }

    pub fn len_vehicle_types(&self) -> usize {
        self.vehicle_types.len()
    }

    pub fn len_commodities(&self) -> usize {
        self.commodities.len()
    }

    pub fn commodity(&self, key: &V::K) -> Option<&CommodityData<V>> {
        self.commodities.get_by_key(key)
    }

    pub fn len_transports(&self) -> usize {
        self.transports.len()
    }

    pub fn transport(&self, key: &V::T) -> Option<&TransportData<V>> {
        self.transports.get_by_key(key)
    }
}
