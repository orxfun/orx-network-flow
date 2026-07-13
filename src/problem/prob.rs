use crate::IdxMap;
use crate::commodities::{Commodities, Commodity, CommodityData};
use crate::common_ds::SortedKeyMap;
use crate::costs::Costs;
use crate::networks::{AoaWaitNw, AoaWaitNwSettings, AonWaitNw, AonWaitNwSettings, GraphStats};
use crate::problem::connectivity::Connectivity;
use crate::problem::variant::Variant;
use crate::spaces::{Space, SpaceData, SpaceTime, Spaces};
use crate::time_bounds::TimeBounds;
use crate::transports::{Transport, TransportData, Transports};
use crate::vehicle_types::{VehicleType, VehicleTypes};
use crate::vehicles::{Vehicle, VehicleData, Vehicles};
use alloc::vec::Vec;

pub struct Problem<V: Variant> {
    pub spaces: Spaces<V>,
    pub vehicle_types: VehicleTypes<V>,
    pub vehicles: Vehicles<V>,
    pub commodities: Commodities<V>,
    pub transports: Transports<V>,
    pub connectivity: Connectivity,
    pub costs: Costs<V>,
    pub time_bounds: TimeBounds,
    pub ori_sorted_commodities: SortedKeyMap<Space, Vec<Commodity>>,
    pub des_sorted_commodities: SortedKeyMap<Space, Vec<Commodity>>,
    pub ori_des_sorted_transports: SortedKeyMap<Space, SortedKeyMap<Space, Vec<Transport>>>,
    pub des_ori_sorted_transports: SortedKeyMap<Space, SortedKeyMap<Space, Vec<Transport>>>,
    pub sorted_ro_commodities: IdxMap<SpaceTime, Vec<Commodity>, usize>,
    pub sorted_dd_commodities: IdxMap<SpaceTime, Vec<Commodity>, usize>,
}

impl<V: Variant> Problem<V> {
    // len
    pub fn len_spaces(&self) -> usize {
        self.spaces.len()
    }

    pub fn len_vehicle_types(&self) -> usize {
        self.vehicle_types.len()
    }

    pub fn len_vehicles(&self) -> usize {
        self.vehicles.len()
    }

    pub fn len_commodities(&self) -> usize {
        self.commodities.len()
    }

    pub fn len_transports(&self) -> usize {
        self.transports.len()
    }

    // get index

    pub fn space_idx(&self, key: &V::S) -> Option<Space> {
        self.spaces.get_ind_by_key(key)
    }

    pub fn commodity_ind(&self, key: &V::K) -> Option<Commodity> {
        self.commodities.get_ind_by_key(key)
    }

    pub fn transport_ind(&self, key: &V::T) -> Option<Transport> {
        self.transports.get_ind_by_key(key)
    }

    pub fn vehicle_type_ind(&self, key: &V::W) -> Option<VehicleType> {
        self.vehicle_types.get_ind_by_key(key)
    }

    pub fn vehicle_ind(&self, key: &V::V) -> Option<Vehicle> {
        self.vehicles.get_ind_by_key(key)
    }

    // get by key

    pub fn commodity(&self, key: &V::K) -> Option<&CommodityData<V>> {
        self.commodities.get_by_key(key)
    }

    pub fn transport(&self, key: &V::T) -> Option<&TransportData<V>> {
        self.transports.get_by_key(key)
    }

    // get key

    pub(crate) fn space_key(&self, idx: Space) -> &V::S {
        self.spaces.key(idx).expect("validated problem")
    }

    pub fn commodity_key(&self, idx: Commodity) -> &V::K {
        self.commodities.key(idx).expect("validated problem")
    }

    // get by idx

    pub(crate) fn space_by_idx(&self, s: Space) -> &SpaceData<V> {
        self.spaces.get_by_idx(s).expect("validated problem")
    }

    pub fn commodity_by_idx(&self, c: Commodity) -> &CommodityData<V> {
        self.commodities.get_by_idx(c).expect("validated problem")
    }

    pub(crate) fn transport_by_idx(&self, t: Transport) -> &TransportData<V> {
        self.transports.get_by_idx(t).expect("validated problem")
    }

    pub(crate) fn vehicle_by_idx(&self, t: Vehicle) -> &VehicleData {
        self.vehicles.get_by_idx(t).expect("validated problem")
    }

    // networks

    pub fn construct_aon_wait_nw(&self, settings: AonWaitNwSettings) -> AonWaitNw<'_, V> {
        AonWaitNw::construct(self, settings)
    }

    pub fn construct_aoa_wait_nw(&self, settings: AoaWaitNwSettings) -> AoaWaitNw<'_, V> {
        AoaWaitNw::construct(self, settings)
    }

    pub fn compute_stats_aon_wait_nw(&self, settings: AonWaitNwSettings) -> GraphStats {
        AonWaitNw::compute_stats(self, settings)
    }

    pub fn compute_stats_aoa_wait_nw(&self, settings: AoaWaitNwSettings) -> GraphStats {
        AoaWaitNw::compute_stats(self, settings)
    }
}
