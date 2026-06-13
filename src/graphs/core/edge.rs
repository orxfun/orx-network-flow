use crate::graphs::{Edge, VIdx};

#[derive(derive_new::new)]
pub struct EdgeCore<De> {
    tail: VIdx,
    head: VIdx,
    data: De,
}

impl<De> EdgeCore<De> {
    pub fn with_data<E>(&self, data: E) -> EdgeCore<E> {
        EdgeCore {
            tail: self.tail,
            head: self.head,
            data,
        }
    }
}

impl<E> Edge for &EdgeCore<E> {
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
