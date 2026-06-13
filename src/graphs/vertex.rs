use crate::{graphs::EIdx, impl_idx, impl_vec_of_idx};
use orx_priority_queue::HasIndex;

impl_idx!(VIdx, VertexRange);
impl_vec_of_idx!(VIdx, VertexRange, VecVertex);

impl HasIndex for VIdx {
    #[inline(always)]
    fn index(&self) -> usize {
        self.0
    }
}

pub trait Vertex {
    type Data;

    fn data(&self) -> &Self::Data;

    fn out_edges(&self) -> impl Iterator<Item = EIdx>;

    fn in_edges(&self) -> impl Iterator<Item = EIdx>;

    fn len_out_edges(&self) -> usize;

    fn len_in_edges(&self) -> usize;
}
