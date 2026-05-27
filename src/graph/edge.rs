use crate::graph::vertex::VIdx;
use crate::{impl_idx, impl_vec_of_idx};

impl_idx!(EIdx);
impl_vec_of_idx!(EIdx, VecEdge);

pub struct Edge<E> {
    tail: VIdx,
    head: VIdx,
    data: E,
}

impl<E> Edge<E> {
    pub fn new(tail: VIdx, head: VIdx, data: E) -> Self {
        Self { tail, head, data }
    }

    pub fn tail(&self) -> VIdx {
        self.tail
    }

    pub fn head(&self) -> VIdx {
        self.head
    }

    pub fn data(&self) -> &E {
        &self.data
    }
}
