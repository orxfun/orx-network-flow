use crate::graph::vertex::VIdx;
use core::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct EIdx(pub(super) usize);

impl Display for EIdx {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "e{}", self.0)
    }
}

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
