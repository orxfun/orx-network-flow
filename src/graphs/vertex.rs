use crate::{graphs::EIdx, impl_idx, impl_vec_of_idx};

impl_idx!(VIdx);
impl_vec_of_idx!(VIdx, VecVertex);

pub trait Vertex {
    type Data;

    fn data(&self) -> &Self::Data;

    fn out_edges(&self) -> impl ExactSizeIterator<Item = EIdx>;

    fn in_edges(&self) -> impl ExactSizeIterator<Item = EIdx>;

    fn len_out_edges(&self) -> usize {
        self.out_edges().len()
    }

    fn len_in_edges(&self) -> usize {
        self.in_edges().len()
    }
}
