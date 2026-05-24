use crate::{flow_units::FlowUnit, std_utils::MapKey};

pub trait Variant {
    /// Space key
    type S: MapKey;

    /// Commodity key
    type K: MapKey;

    /// Flow unit
    type F: FlowUnit;
}
