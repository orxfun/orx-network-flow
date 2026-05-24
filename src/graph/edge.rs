pub struct Edge<E> {
    tail: usize,
    head: usize,
    data: E,
}

impl<E> Edge<E> {
    pub fn new(tail: usize, head: usize, data: E) -> Self {
        Self { tail, head, data }
    }
}
