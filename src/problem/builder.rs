use crate::costs::{EarlinessCost, LatenessCost, LostRevenue, TransportCost};
use crate::problem::Problem;
use crate::problem::variant::Variant;
use crate::space_time::SpaceTime;
use crate::time::Time;
use crate::time_bounds::{ConnTimeBounds, LatenessEarlinessBounds};

pub struct ProblemBuilder<V: Variant>(Problem<V>);

impl<V: Variant> Default for ProblemBuilder<V> {
    fn default() -> Self {
        Self(Problem {
            spaces: Default::default(),
            vehicle_types: Default::default(),
            vehicles: Default::default(),
            commodities: Default::default(),
            transports: Default::default(),
            costs: Default::default(),
            time_bounds: Default::default(),
            ori_spaces: Default::default(),
            des_spaces: Default::default(),
            ori_commodities: Default::default(),
            des_commodities: Default::default(),
        })
    }
}

impl<V: Variant> ProblemBuilder<V> {
    // create and complete
    pub fn new() -> Self {
        Default::default()
    }

    pub fn finish(self) -> Problem<V> {
        self.0
    }

    // build

    pub fn push_commodity(
        &mut self,
        commodity_key: V::K,
        origin: V::S,
        ready_time: impl Into<Time>,
        destination: V::S,
        due_time: impl Into<Time>,
        amount: V::F,
    ) {
        let ori_space = self.0.spaces.push(origin);
        self.0.ori_spaces.insert(ori_space);
        let ori = SpaceTime::new(ori_space, ready_time.into());

        let des_space = self.0.spaces.push(destination);
        self.0.des_spaces.insert(des_space);
        let des = SpaceTime::new(des_space, due_time.into());

        _ = self.0.commodities.push(commodity_key, ori, des, amount);
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

        let ori_space = self.0.spaces.push(origin);
        let ori = SpaceTime::new(ori_space, departure_time.into());

        let des_space = self.0.spaces.push(destination);
        let des = SpaceTime::new(des_space, arrival_time.into());

        _ = self
            .0
            .transports
            .push(transport_key, vehicle, ori, des, capacity);
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

    pub fn min_conn_time(&mut self) -> &mut ConnTimeBounds {
        &mut self.0.time_bounds.min_conn_time
    }

    pub fn max_conn_time(&mut self) -> &mut ConnTimeBounds {
        &mut self.0.time_bounds.max_conn_time
    }

    pub fn max_lateness(&mut self) -> &mut LatenessEarlinessBounds {
        &mut self.0.time_bounds.max_lateness
    }

    pub fn max_earliness(&mut self) -> &mut LatenessEarlinessBounds {
        &mut self.0.time_bounds.max_earliness
    }

    pub fn max_waiting(&mut self) -> &mut LatenessEarlinessBounds {
        &mut self.0.time_bounds.max_waiting
    }
}
