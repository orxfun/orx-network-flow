use crate::{cost::Cost, flow_units::FlowUnit, std_utils::MapKey};

pub trait Variant {
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
