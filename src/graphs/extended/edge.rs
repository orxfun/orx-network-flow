use crate::graphs::EIdx;

pub struct OriEdge<E> {
    pub(super) core_e: EIdx,
    pub(super) ext_data: E,
}
