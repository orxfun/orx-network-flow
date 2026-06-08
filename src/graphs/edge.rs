use crate::{graphs::VIdx, impl_idx, impl_vec_of_idx};

impl_idx!(EIdx);
impl_vec_of_idx!(EIdx, VecEdge);

pub trait Edge {
    type Data;

    fn data(&self) -> &Self::Data;

    fn tail(&self) -> VIdx;

    fn head(&self) -> VIdx;
}
