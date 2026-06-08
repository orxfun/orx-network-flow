enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct EitherIter<L, R>(Either<L, R>)
where
    L: Iterator,
    R: Iterator<Item = L::Item>;

impl<L, R> EitherIter<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    pub fn new_left(iter: L) -> Self {
        Self(Either::Left(iter))
    }

    pub fn new_right(iter: R) -> Self {
        Self(Either::Right(iter))
    }
}

impl<L, R> Iterator for EitherIter<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Either::Left(x) => x.next(),
            Either::Right(y) => y.next(),
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Either::Left(x) => x.size_hint(),
            Either::Right(y) => y.size_hint(),
        }
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self.0 {
            Either::Left(x) => x.fold(init, f),
            Either::Right(y) => y.fold(init, f),
        }
    }
}

impl<L, R> ExactSizeIterator for EitherIter<L, R>
where
    L: ExactSizeIterator,
    R: ExactSizeIterator<Item = L::Item>,
{
    #[inline(always)]
    fn len(&self) -> usize {
        match &self.0 {
            Either::Left(x) => x.len(),
            Either::Right(y) => y.len(),
        }
    }
}
