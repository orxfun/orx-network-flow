use crate::graphs::EIdx;

pub struct CoreEdge<Ee> {
    core_eidx: EIdx,
    data: Ee,
}

impl<Ee> CoreEdge<Ee> {
    pub fn new(core_eidx: EIdx, data: Ee) -> Self {
        Self { core_eidx, data }
    }
}
