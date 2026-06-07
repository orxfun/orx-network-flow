use crate::graph::EIdx;
use core::marker::PhantomData;

pub struct ExtEdge<E, Ee> {
    core_eidx: EIdx,
    data: Ee,
    core_data: PhantomData<E>,
}

impl<E, Ee> ExtEdge<E, Ee> {
    pub fn new(core_eidx: EIdx, data: Ee) -> Self {
        Self {
            core_eidx,
            data,
            core_data: PhantomData,
        }
    }
}
