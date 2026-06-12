use crate::commodities::Commodity;
use crate::costs::{EarlinessCost, LatenessCost, LostRevenue, TransportCost};
use crate::problem::Problem;
use crate::problem::connectivity::{
    SpatialConnectivity, SpatialConnectivityBuilder, TemporalConnectivity,
    TemporalConnectivityBuilder,
};
use crate::problem::variant::Variant;
use crate::space_time::SpaceTime;
use crate::spaces::{Coordinate, Geocode, Location, Space, SpaceData};
use crate::time::Time;
use crate::time_bounds::{ArrivalTimeBoundsBuilder, DepartureTimeBoundsBuilder};
use crate::transports::Transport;
use core::marker::PhantomData;

pub trait ProblemBuilderState {}

pub struct DefiningSpaces;
impl ProblemBuilderState for DefiningSpaces {}

pub struct DefiningProblem;
impl ProblemBuilderState for DefiningProblem {}

pub struct ProblemBuilder<V: Variant, S: ProblemBuilderState>(Problem<V>, PhantomData<S>);

impl<V: Variant> ProblemBuilder<V, DefiningSpaces> {
    // create and complete
    pub fn new() -> Self {
        Self(
            Problem {
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
                sorted_transport_origins: Default::default(),
                sorted_commodity_origins: Default::default(),
                sorted_commodity_destinations: Default::default(),
            },
            PhantomData,
        )
    }

    pub fn spaces(self) -> ProblemBuilder<V, DefiningProblem> {
        ProblemBuilder(self.0, PhantomData)
    }

    pub fn with_basic_spaces(
        mut self,
        spaces: impl IntoIterator<Item = V::S>,
    ) -> ProblemBuilder<V, DefiningProblem> {
        for s in spaces {
            self.0.spaces.push(s, SpaceData::new(Location::Basic));
        }
        ProblemBuilder(self.0, PhantomData)
    }

    pub fn with_euclidean_spaces(
        mut self,
        spaces: impl IntoIterator<Item = (V::S, f64, f64)>,
    ) -> ProblemBuilder<V, DefiningProblem> {
        for (s, x, y) in spaces {
            self.0
                .spaces
                .push(s, SpaceData::new(Location::Euclidean(Coordinate { x, y })));
        }
        ProblemBuilder(self.0, PhantomData)
    }

    pub fn with_geographic_spaces(
        mut self,
        spaces: impl IntoIterator<Item = (V::S, f64, f64)>,
    ) -> ProblemBuilder<V, DefiningProblem> {
        for (s, lat, lon) in spaces {
            self.0.spaces.push(
                s,
                SpaceData::new(Location::Geographic(Geocode { lat, lon })),
            );
        }
        ProblemBuilder(self.0, PhantomData)
    }
}

impl<V: Variant> ProblemBuilder<V, DefiningProblem> {
    pub fn finish(mut self) -> Problem<V> {
        // sort ori and des commodities by ready time and due time

        let mut ori_sorted_commodities = Default::default();
        core::mem::swap(
            &mut ori_sorted_commodities,
            &mut self.0.ori_sorted_commodities,
        );

        let mut des_sorted_commodities = Default::default();
        core::mem::swap(
            &mut des_sorted_commodities,
            &mut self.0.des_sorted_commodities,
        );

        let sort_key = |c: &Commodity| self.0.commodity_by_idx(*c).origin().time();
        for x in ori_sorted_commodities.values_mut() {
            x.sort_by_key(&sort_key);
        }

        let sort_key = |c: &Commodity| self.0.commodity_by_idx(*c).destination().time();
        for x in des_sorted_commodities.values_mut() {
            x.sort_by_key(&sort_key);
        }

        self.0.ori_sorted_commodities = ori_sorted_commodities;
        self.0.des_sorted_commodities = des_sorted_commodities;

        // sort ori&des and des&ori transports by departure time

        let mut ori_des_sorted_transports = Default::default();
        core::mem::swap(
            &mut ori_des_sorted_transports,
            &mut self.0.ori_des_sorted_transports,
        );

        let mut des_ori_sorted_transports = Default::default();
        core::mem::swap(
            &mut des_ori_sorted_transports,
            &mut self.0.des_ori_sorted_transports,
        );

        let sort_key = |t: &Transport| self.0.transport_by_idx(*t).origin().time();

        for des_sorted_transports in ori_des_sorted_transports.values_mut() {
            for x in des_sorted_transports.values_mut() {
                x.sort_by_key(&sort_key);
            }
        }

        for ori_sorted_transports in des_ori_sorted_transports.values_mut() {
            for x in ori_sorted_transports.values_mut() {
                x.sort_by_key(&sort_key);
            }
        }

        self.0.ori_des_sorted_transports = ori_des_sorted_transports;
        self.0.des_ori_sorted_transports = des_ori_sorted_transports;

        // sorted transport origins

        self.0.sorted_transport_origins =
            self.0.ori_des_sorted_transports.keys().copied().collect();
        self.0.sorted_transport_origins.sort();

        // sorted commodity origins & destinations

        self.0.sorted_commodity_origins = self.0.ori_sorted_commodities.keys().copied().collect();
        self.0.sorted_commodity_origins.sort();

        self.0.sorted_commodity_destinations =
            self.0.des_sorted_commodities.keys().copied().collect();
        self.0.sorted_commodity_destinations.sort();

        // finish

        self.0
    }

    // build

    fn space_unwrap(&self, key: &V::S) -> Space {
        match self.0.spaces.get_ind_by_key(key) {
            Some(s) => s,
            None => panic!("Missing space '{key}'"),
        }
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

        let commodity = self.0.commodities.push(commodity_key, ori, des, amount);

        self.0
            .ori_sorted_commodities
            .entry(ori_space)
            .or_default()
            .push(commodity);
        self.0
            .des_sorted_commodities
            .entry(des_space)
            .or_default()
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
        let vehicle_type = self.0.vehicle_types.push(vehicle_type_key);
        let vehicle = self.0.vehicles.push(vehicle_key, vehicle_type);

        let ori_space = self.space_unwrap(&origin);
        let ori = SpaceTime::new(ori_space, departure_time.into());

        let des_space = self.space_unwrap(&destination);
        let des = SpaceTime::new(des_space, arrival_time.into());

        let transport = self
            .0
            .transports
            .push(transport_key, vehicle, ori, des, capacity);

        self.0
            .ori_des_sorted_transports
            .entry(ori_space)
            .or_default()
            .entry(des_space)
            .or_default()
            .push(transport);
        self.0
            .des_ori_sorted_transports
            .entry(des_space)
            .or_default()
            .entry(ori_space)
            .or_default()
            .push(transport);
    }

    pub fn spatial_connectivity(&mut self) -> SpatialConnectivityBuilder<'_, V> {
        let spatial =
            unsafe { &mut *(&mut self.0.connectivity.spatial as *mut SpatialConnectivity) };
        SpatialConnectivityBuilder::new(&self.0, spatial)
    }

    pub fn temporal_connectivity(&mut self) -> TemporalConnectivityBuilder<'_, V> {
        let temporal =
            unsafe { &mut *(&mut self.0.connectivity.temporal as *mut TemporalConnectivity) };
        TemporalConnectivityBuilder::new(&self.0, temporal)
    }

    // costs

    pub fn earliness_cost(&mut self) -> &mut EarlinessCost<V> {
        &mut self.0.costs.earliness
    }

    pub fn lateness_cost(&mut self) -> &mut LatenessCost<V> {
        &mut self.0.costs.lateness
    }

    pub fn lost_revenue_cost(&mut self) -> &mut LostRevenue<V> {
        &mut self.0.costs.lost_revenue
    }

    pub fn transport_cost(&mut self) -> &mut TransportCost<V> {
        &mut self.0.costs.transport
    }

    // time bounds

    pub fn max_lateness(&mut self) -> ArrivalTimeBoundsBuilder<'_, V> {
        ArrivalTimeBoundsBuilder::lateness(&mut self.0)
    }

    pub fn max_earliness(&mut self) -> ArrivalTimeBoundsBuilder<'_, V> {
        ArrivalTimeBoundsBuilder::earliness(&mut self.0)
    }

    pub fn max_waiting(&mut self) -> DepartureTimeBoundsBuilder<'_, V> {
        DepartureTimeBoundsBuilder::new(&mut self.0)
    }
}
