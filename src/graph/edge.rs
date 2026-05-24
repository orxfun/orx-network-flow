pub struct Edge<E> {
    tail: usize,
    head: usize,
    data: E,
}

impl<E> Edge<E> {
    pub fn new(tail: usize, head: usize, data: E) -> Self {
        Self { tail, head, data }
    }

    pub fn tail(&self) -> usize {
        self.tail
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn data(&self) -> &E {
        &self.data
    }
}
