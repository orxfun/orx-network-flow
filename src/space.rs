use crate::{impl_from_usize, indices::Index};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Space(usize);

impl_from_usize!(Space);
impl Index for Space {}
