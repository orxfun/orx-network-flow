use crate::{impl_from_usize, indices::Index};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Commodity(usize);

impl_from_usize!(Commodity);
impl Index for Commodity {}
