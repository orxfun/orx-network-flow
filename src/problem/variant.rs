use crate::{amounts::Amount, std_utils::MapKey};

pub trait Variant {
    /// Space key
    type S: MapKey;

    /// Commodity key
    type K: MapKey;

    type A: Amount;
}
