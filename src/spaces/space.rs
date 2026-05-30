use crate::{impl_idx, impl_vec_of_idx};

impl_idx!(Space);
impl_vec_of_idx!(Space, VecSpace);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpaceOd {
    pub ori: Space,
    pub des: Space,
}
