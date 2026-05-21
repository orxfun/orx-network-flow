use crate::std_utils::{Map, MapKey};
use alloc::vec::Vec;

pub struct Spaces<D: MapKey> {
    data: Vec<D>,
    map_to_idx: Map<D, usize>,
}

impl<D: MapKey> Spaces<D> {
    //
}
