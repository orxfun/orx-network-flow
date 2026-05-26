use crate::{flow_units::FlowUnit, std_utils::MapKey};

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
}
