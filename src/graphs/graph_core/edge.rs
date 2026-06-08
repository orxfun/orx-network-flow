use crate::graphs::{Edge, VIdx};

#[derive(derive_new::new)]
pub struct EdgeCore<E> {
    tail: VIdx,
    head: VIdx,
    data: E,
}

impl<E> Edge for EdgeCore<E> {
    type Data = E;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn tail(&self) -> VIdx {
        self.tail
    }

    fn head(&self) -> VIdx {
        self.head
    }
}
