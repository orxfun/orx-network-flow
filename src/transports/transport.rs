use crate::{impl_from_usize, indices::Index};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transport(usize);

impl_from_usize!(Transport);
impl Index for Transport {}
