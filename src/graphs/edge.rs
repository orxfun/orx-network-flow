use crate::{graphs::VIdx, impl_idx, impl_vec_of_idx};

impl_idx!(EIdx, EdgeRange);
impl_vec_of_idx!(EIdx, EdgeRange, VecEdge);

pub trait Edge {
    type Data;

    fn data(&self) -> &Self::Data;

    fn tail(&self) -> VIdx;

    fn head(&self) -> VIdx;
}
