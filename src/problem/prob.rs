use crate::Graph;
use crate::commodities::Commodities;
use crate::commodities::Commodity;
use crate::commodities::CommodityData;
use crate::costs::Costs;
use crate::costs::EarlinessCost;
use crate::costs::LatenessCost;
use crate::costs::LostRevenue;
use crate::costs::TransportCost;
use crate::graph_builders::AonGraph;
use crate::graph_builders::build_aon_graph;
use crate::problem::variant::Variant;
use crate::spaces::Spaces;
use crate::time_bounds::ConnTimeBounds;
use crate::time_bounds::LatenessEarlinessBounds;
use crate::time_bounds::TimeBounds;
use crate::transports::Transport;
use crate::transports::TransportData;
use crate::transports::Transports;
use crate::vehicle_types::VehicleTypes;
use crate::vehicles::Vehicle;
use crate::vehicles::VehicleData;
use crate::vehicles::Vehicles;

pub struct Problem<V: Variant> {
    pub(super) spaces: Spaces<V::S>,
    pub(super) vehicle_types: VehicleTypes<V>,
    pub(super) vehicles: Vehicles<V>,
    pub(super) commodities: Commodities<V>,
    pub(super) transports: Transports<V>,
    pub(super) costs: Costs<V>,
    pub(super) time_bounds: TimeBounds,
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

    // costs

    pub fn earliness_cost(&self) -> &EarlinessCost<V> {
        &self.costs.earliness
    }

    pub fn lateness_cost(&self) -> &LatenessCost<V> {
        &self.costs.lateness
    }

    pub fn lost_revenue_cost(&self) -> &LostRevenue<V> {
        &self.costs.lost_revenue
    }

    pub fn transport_cost(&self) -> &TransportCost<V> {
        &self.costs.transport
    }

    // time bounds

    pub fn min_conn_time(&self) -> &ConnTimeBounds {
        &self.time_bounds.min_conn_time
    }

    pub fn max_conn_time(&self) -> &ConnTimeBounds {
        &self.time_bounds.max_conn_time
    }

    pub fn max_lateness(&self) -> &LatenessEarlinessBounds {
        &self.time_bounds.max_lateness
    }

    pub fn max_earliness(&self) -> &LatenessEarlinessBounds {
        &self.time_bounds.max_earliness
    }

    pub fn max_waiting(&self) -> &LatenessEarlinessBounds {
        &self.time_bounds.max_waiting
    }

    // graphs

    pub fn build_aon_graph(&self) -> AonGraph {
        build_aon_graph(self)
    }
}
