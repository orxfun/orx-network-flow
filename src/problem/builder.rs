use crate::common_ds::{SortedKeyMap, SortedKeyMapBuilder};
use crate::costs::Costs;
use crate::costs::{EarlinessCost, LatenessCost, LostRevenue, LostRevenueBuilder, TransportCost};
use crate::problem::connectivity::{
    Connectivity, SpatialConnectivity, SpatialConnectivityBuilder, TemporalConnectivity,
    TemporalConnectivityBuilder,
};
use crate::spaces::Spaces;
use crate::spaces::{Coordinate, Geocode, LocationDepr, SpaceData};
use crate::time_bounds::TimeBounds;
use crate::time_bounds::{ArrivalTimeBoundsBuilder, DepartureTimeBoundsBuilder};
use crate::transports::Transports;
use crate::utils::std_utils::Map;
use crate::vehicle_types::VehicleTypes;
use crate::vehicles::Vehicles;
use crate::{Commodities, IdxMap, NoLocation};
use crate::{Commodity, Problem, Space, SpaceTime, Time, Transport, Variant};
use alloc::vec::Vec;

pub struct ProblemBuilder<V: Variant> {
    spaces: Spaces<V>,
    vehicle_types: VehicleTypes<V>,
    vehicles: Vehicles<V>,
    commodities: Commodities<V>,
    transports: Transports<V>,
    connectivity: Connectivity,
    costs: Costs<V>,
    time_bounds: TimeBounds,
    ori_sorted_commodities: SortedKeyMapBuilder<Space, Vec<Commodity>>,
    des_sorted_commodities: SortedKeyMapBuilder<Space, Vec<Commodity>>,
    ori_des_sorted_transports: Map<Space, SortedKeyMapBuilder<Space, Vec<Transport>>>,
    des_ori_sorted_transports: Map<Space, SortedKeyMapBuilder<Space, Vec<Transport>>>,
    sorted_ro_commodities: IdxMap<SpaceTime, Vec<Commodity>, usize>,
    sorted_dd_commodities: IdxMap<SpaceTime, Vec<Commodity>, usize>,
}

impl<V: Variant> ProblemBuilder<V> {
    pub fn new() -> Self {
        Self {
            spaces: Default::default(),
            vehicle_types: Default::default(),
            vehicles: Default::default(),
            commodities: Default::default(),
            transports: Default::default(),
            connectivity: Default::default(),
            costs: Default::default(),
            time_bounds: Default::default(),
            ori_sorted_commodities: Default::default(),
            des_sorted_commodities: Default::default(),
            ori_des_sorted_transports: Default::default(),
            des_ori_sorted_transports: Default::default(),
            sorted_ro_commodities: Default::default(),
            sorted_dd_commodities: Default::default(),
        }
    }

    // build

    pub fn push_space(&mut self, key: V::S, location: V::L) {
        //
    }
}

impl<V: Variant<L = NoLocation>> ProblemBuilder<V> {
    pub fn push_basic_space(&mut self, key: V::S) {
        // self.spaces.push(key, data)
    }
}
