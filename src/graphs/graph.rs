use crate::graphs::{EIdx, VIdx};

pub trait Graph {
    type V<'a>
    where
        Self: 'a;

    type E<'a>
    where
        Self: 'a;

    fn vertex(&self, v: VIdx) -> Self::V<'_>;

    fn edge(&self, e: EIdx) -> Self::E<'_>;
}
