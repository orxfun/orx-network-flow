use crate::costs::{EarlinessCost, LatenessCost, LostRevenue, TransportCost};
use crate::problem::Problem;
use crate::problem::variant::Variant;
use crate::space_time::SpaceTime;
use crate::time::Time;

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
        })
    }
}

impl<V: Variant> ProblemBuilder<V> {
    pub fn new() -> Self {
        Default::default()
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
        let ori_space = self.0.spaces.push(origin);
        let ori = SpaceTime::new(ori_space, ready_time.into());

        let des_space = self.0.spaces.push(destination);
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

    pub fn earliness_cost(&mut self) -> &mut EarlinessCost<V> {
        self.0.costs.earliness_mut()
    }

    pub fn lateness_cost(&mut self) -> &mut LatenessCost<V> {
        self.0.costs.lateness_mut()
    }

    pub fn lost_revenue_cost(&mut self) -> &mut LostRevenue<V> {
        self.0.costs.lost_revenue_mut()
    }

    pub fn transport_cost(&mut self) -> &mut TransportCost<V> {
        self.0.costs.transport_mut()
    }

    pub fn finish(self) -> Problem<V> {
        self.0
    }
}
