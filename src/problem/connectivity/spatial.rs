use crate::{Variant, std_utils::Set};
use std::boxed::Box;

pub struct SpatialConnectivity<V: Variant> {
    /// It is not allowed to connect transport a->b with b->c
    /// if (a,b,c) is in the taboo_set.
    taboo_set: Set<(V::S, V::S, V::S)>,
    can_connect: Box<V>,
}
