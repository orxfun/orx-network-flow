use crate::{cost::Cost, flow_units::FlowUnit, spaces::Location, utils::std_utils::MapKey};

pub trait Variant: Clone + Copy + Default + 'static {
    /// Location kind
    type L: Location;

    /// Space key
    type S: MapKey;

    /// Commodity key
    type K: MapKey;

    /// Vehicle type key
    type W: MapKey;

    /// Vehicle key
    type V: MapKey;

    /// Transport key
    type T: MapKey;

    /// Flow unit
    type F: FlowUnit;

    /// Cost (objective) unit
    type C: Cost;

    fn chargeable_flow(flow: Self::F) -> Self::C;
}
