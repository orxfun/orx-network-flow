use crate::common_ds::{SortedKeyMap, SortedKeyMapBuilder};
use crate::costs::Costs;
use crate::costs::{EarlinessCost, LatenessCost, LostRevenue, LostRevenueBuilder, TransportCost};
use crate::problem::connectivity::{
    Connectivity, SpatialConnectivity, SpatialConnectivityBuilder, TemporalConnectivity,
    TemporalConnectivityBuilder,
};
use crate::problem::space_connectivity::{SpaceConnectivity, SpaceConnectivityBuilder};
use crate::spaces::{Coordinate, Geocode, SpaceData};
use crate::spaces::{Geographical, Spaces};
use crate::time_bounds::TimeBounds;
use crate::time_bounds::{ArrivalTimeBoundsBuilder, DepartureTimeBoundsBuilder};
use crate::transports::Transports;
use crate::utils::std_utils::Map;
use crate::vehicle_types::VehicleTypes;
use crate::vehicles::Vehicles;
use crate::{Commodities, Euclidean, IdxMap, NoLocation};
use crate::{Commodity, Problem, Space, SpaceTime, Time, Transport, Variant};
use alloc::vec::Vec;

pub struct ProblemBuilder<V: Variant> {
    spaces: Spaces<V>,
    vehicle_types: VehicleTypes<V>,
    vehicles: Vehicles<V>,
    commodities: Commodities<V>,
    transports: Transports<V>,
    connectivity: Connectivity<V>,
    costs: Costs<V>,
    time_bounds: TimeBounds,
    ori_sorted_commodities: SortedKeyMapBuilder<Space, Vec<Commodity>>,
    des_sorted_commodities: SortedKeyMapBuilder<Space, Vec<Commodity>>,
    ori_des_sorted_transports: Map<Space, SortedKeyMapBuilder<Space, Vec<Transport>>>,
    des_ori_sorted_transports: Map<Space, SortedKeyMapBuilder<Space, Vec<Transport>>>,
    sorted_ro_commodities: IdxMap<SpaceTime, Vec<Commodity>, usize>,
    sorted_dd_commodities: IdxMap<SpaceTime, Vec<Commodity>, usize>,
}

impl<V: Variant<L = NoLocation>> ProblemBuilder<V> {
    pub fn push_basic_space(&mut self, key: V::S) -> Space {
        self.spaces.push(key, SpaceData::new(NoLocation))
    }
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

    pub fn push_space(&mut self, key: V::S, location: V::L) -> Space {
        self.spaces.push(key, SpaceData::new(location))
    }

    pub fn push_commodity(
        &mut self,
        commodity_key: V::K,
        origin: V::S,
        ready_time: impl Into<Time>,
        destination: V::S,
        due_time: impl Into<Time>,
        amount: V::F,
    ) {
        let ori_space = self.space_unwrap(&origin);
        let ori = SpaceTime::new(ori_space, ready_time.into());

        let des_space = self.space_unwrap(&destination);
        let des = SpaceTime::new(des_space, due_time.into());

        let commodity = self.commodities.push(commodity_key, ori, des, amount);

        self.ori_sorted_commodities
            .get_or_add_default_mut(ori_space)
            .push(commodity);
        self.des_sorted_commodities
            .get_or_add_default_mut(des_space)
            .push(commodity);
    }

    pub fn push_transport(
        &mut self,
        transport_key: V::T,
        vehicle_key: V::V,
        vehicle_type_key: V::W,
        origin: V::S,
        departure_time: impl Into<Time>,
        destination: V::S,
        arrival_time: impl Into<Time>,
        capacity: V::F,
    ) {
        let vehicle_type = self.vehicle_types.push(vehicle_type_key);
        let vehicle = self.vehicles.push(vehicle_key, vehicle_type);

        let ori_space = self.space_unwrap(&origin);
        let ori = SpaceTime::new(ori_space, departure_time.into());

        let des_space = self.space_unwrap(&destination);
        let des = SpaceTime::new(des_space, arrival_time.into());

        let transport = self
            .transports
            .push(transport_key, vehicle, ori, des, capacity);

        self.ori_des_sorted_transports
            .entry(ori_space)
            .or_default()
            .get_or_add_default_mut(des_space)
            .push(transport);
        self.des_ori_sorted_transports
            .entry(des_space)
            .or_default()
            .get_or_add_default_mut(ori_space)
            .push(transport);
    }

    pub fn spatial_connectivity(&mut self) -> SpaceConnectivityBuilder<'_, V> {
        let spaces = unsafe { &*(&self.spaces as *const Spaces<V>) };
        let space = unsafe { &mut *(&mut self.connectivity.space as *mut SpaceConnectivity<V>) };
        SpaceConnectivityBuilder::new(spaces, space)
    }

    pub fn temporal_connectivity(&mut self) -> TemporalConnectivityBuilder<'_, V> {
        let spaces = unsafe { &*(&self.spaces as *const Spaces<V>) };
        let temporal =
            unsafe { &mut *(&mut self.connectivity.temporal as *mut TemporalConnectivity) };
        TemporalConnectivityBuilder::new(spaces, temporal)
    }

    // costs

    pub fn earliness_cost(&mut self) -> &mut EarlinessCost<V> {
        &mut self.costs.earliness
    }

    pub fn lateness_cost(&mut self) -> &mut LatenessCost<V> {
        &mut self.costs.lateness
    }

    pub fn lost_revenue_cost<'a>(&'a mut self) -> LostRevenueBuilder<'a, V> {
        let commodities = unsafe { &*(&self.commodities as *const Commodities<V>) };
        let lost_revenue = unsafe { &mut *(&mut self.costs.lost_revenue as *mut LostRevenue<_>) };
        LostRevenueBuilder::new(commodities, lost_revenue)
    }

    pub fn transport_cost(&mut self) -> &mut TransportCost<V> {
        &mut self.costs.transport
    }

    // time bounds

    pub fn max_lateness(&mut self) -> ArrivalTimeBoundsBuilder<'_, V> {
        let spaces = unsafe { &*(&self.spaces as *const Spaces<V>) };
        let commodities = unsafe { &*(&self.commodities as *const Commodities<V>) };
        let time_bounds = unsafe { &mut *(&mut self.time_bounds as *mut TimeBounds) };
        ArrivalTimeBoundsBuilder::lateness(spaces, commodities, time_bounds)
    }

    pub fn max_earliness(&mut self) -> ArrivalTimeBoundsBuilder<'_, V> {
        let spaces = unsafe { &*(&self.spaces as *const Spaces<V>) };
        let commodities = unsafe { &*(&self.commodities as *const Commodities<V>) };
        let time_bounds = unsafe { &mut *(&mut self.time_bounds as *mut TimeBounds) };
        ArrivalTimeBoundsBuilder::earliness(spaces, commodities, time_bounds)
    }

    pub fn max_waiting(&mut self) -> DepartureTimeBoundsBuilder<'_, V> {
        let spaces = unsafe { &*(&self.spaces as *const Spaces<V>) };
        let commodities = unsafe { &*(&self.commodities as *const Commodities<V>) };
        let time_bounds = unsafe { &mut *(&mut self.time_bounds as *mut TimeBounds) };
        DepartureTimeBoundsBuilder::new(spaces, commodities, time_bounds)
    }

    // complete

    pub fn finish(mut self) -> Problem<V> {
        // sort ori and des commodities by ready time and due time

        let commodities = &self.commodities;
        for x in self.ori_sorted_commodities.values_mut() {
            let sort_key = |c: &Commodity| {
                commodities
                    .get_by_idx(*c)
                    .expect("validated problem")
                    .origin()
                    .time()
            };
            x.sort_by_key(&sort_key);
        }

        for x in self.des_sorted_commodities.values_mut() {
            let sort_key = |c: &Commodity| {
                commodities
                    .get_by_idx(*c)
                    .expect("validated problem")
                    .destination()
                    .time()
            };
            x.sort_by_key(&sort_key);
        }

        // sort ori&des and des&ori transports by departure time

        let transports = &self.transports;
        let mut ori_des_sorted_transports = SortedKeyMapBuilder::default();
        for (ori, des_sorted_transports) in self.ori_des_sorted_transports.iter_mut() {
            for x in des_sorted_transports.values_mut() {
                let sort_key = |t: &Transport| {
                    transports
                        .get_by_idx(*t)
                        .expect("validated problem")
                        .origin()
                        .time()
                };
                x.sort_by_key(&sort_key);
            }
            ori_des_sorted_transports.insert(*ori, des_sorted_transports.drain_finished());
        }

        let mut des_ori_sorted_transports = SortedKeyMapBuilder::default();
        for (des, ori_sorted_transports) in self.des_ori_sorted_transports.iter_mut() {
            for x in ori_sorted_transports.values_mut() {
                let sort_key = |t: &Transport| {
                    transports
                        .get_by_idx(*t)
                        .expect("validated problem")
                        .origin()
                        .time()
                };
                x.sort_by_key(&sort_key);
            }
            des_ori_sorted_transports.insert(*des, ori_sorted_transports.drain_finished());
        }

        // sorted ro & dd commodities
        let mut ro_commodities: Map<_, Vec<_>> = Map::default();
        let mut dd_commodities: Map<_, Vec<_>> = Map::default();
        for (c, x) in self.commodities.indices_values() {
            ro_commodities.entry(x.origin()).or_default().push(c);
            dd_commodities.entry(x.destination()).or_default().push(c);
        }

        let mut ro_commodities: Vec<_> = ro_commodities.into_iter().collect();
        ro_commodities.sort();
        for (_, commodities) in &mut ro_commodities {
            commodities.sort();
        }
        self.sorted_ro_commodities = ro_commodities.into_iter().collect();

        let mut dd_commodities: Vec<_> = dd_commodities.into_iter().collect();
        dd_commodities.sort();
        for (_, commodities) in &mut dd_commodities {
            commodities.sort();
        }
        self.sorted_dd_commodities = dd_commodities.into_iter().collect();

        // finish

        Problem {
            spaces: self.spaces,
            vehicle_types: self.vehicle_types,
            vehicles: self.vehicles,
            commodities: self.commodities,
            transports: self.transports,
            connectivity: self.connectivity,
            costs: self.costs,
            time_bounds: self.time_bounds,
            ori_sorted_commodities: self.ori_sorted_commodities.finish(),
            des_sorted_commodities: self.des_sorted_commodities.finish(),
            ori_des_sorted_transports: ori_des_sorted_transports.finish(),
            des_ori_sorted_transports: des_ori_sorted_transports.finish(),
            sorted_ro_commodities: self.sorted_ro_commodities,
            sorted_dd_commodities: self.sorted_dd_commodities,
        }
    }
}

// helpers

impl<V: Variant> ProblemBuilder<V> {
    fn space_unwrap(&self, key: &V::S) -> Space {
        match self.spaces.get_ind_by_key(key) {
            Some(s) => s,
            None => panic!("Missing space '{key}'"),
        }
    }
}
