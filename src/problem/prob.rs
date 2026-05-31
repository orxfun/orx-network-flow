use crate::commodities::Commodities;
use crate::commodities::Commodity;
use crate::commodities::CommodityData;
use crate::costs::Costs;
use crate::graph_builders::activity_on_node::{AonDotGraph, AonGraph, build_aon_graph};
use crate::problem::variant::Variant;
use crate::spaces::Space;
use crate::spaces::Spaces;
use crate::std_utils::Map;
use crate::time_bounds::TimeBounds;
use crate::transports::Transport;
use crate::transports::TransportData;
use crate::transports::Transports;
use crate::vehicle_types::VehicleTypes;
use crate::vehicles::Vehicle;
use crate::vehicles::VehicleData;
use crate::vehicles::Vehicles;
use alloc::vec::Vec;

pub struct Problem<V: Variant> {
    pub spaces: Spaces<V::S>,
    pub vehicle_types: VehicleTypes<V>,
    pub vehicles: Vehicles<V>,
    pub commodities: Commodities<V>,
    pub transports: Transports<V>,
    pub costs: Costs<V>,
    pub time_bounds: TimeBounds,
    pub ori_sorted_commodities: Map<Space, Vec<Commodity>>,
    pub des_sorted_commodities: Map<Space, Vec<Commodity>>,
    pub ori_des_sorted_transports: Map<Space, Map<Space, Vec<Transport>>>,
    pub des_ori_sorted_transports: Map<Space, Map<Space, Vec<Transport>>>,
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

    // get by key

    pub fn commodity(&self, key: &V::K) -> Option<&CommodityData<V>> {
        self.commodities.get_by_key(key)
    }

    pub fn transport(&self, key: &V::T) -> Option<&TransportData<V>> {
        self.transports.get_by_key(key)
    }

    // get by idx

    pub(crate) fn commodity_by_idx(&self, idx: Commodity) -> &CommodityData<V> {
        self.commodities.get_by_idx(idx).expect("validated problem")
    }

    pub(crate) fn transport_by_idx(&self, idx: Transport) -> &TransportData<V> {
        self.transports.get_by_idx(idx).expect("validated problem")
    }

    pub(crate) fn vehicle_by_idx(&self, idx: Vehicle) -> &VehicleData {
        self.vehicles.get_by_idx(idx).expect("validated problem")
    }

    // graphs

    pub fn build_aon_graph(&self) -> AonGraph {
        build_aon_graph(self)
    }
}
