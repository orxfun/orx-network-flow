use crate::graphs::VIdx;

pub struct EdgeCore<E> {
    tail: VIdx,
    head: VIdx,
    data: E,
}

impl<E> EdgeCore<E> {
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
