use crate::graphs::core::{EdgeCore, VertexCore};
use crate::graphs::extended::{edge::OriEdge, vertex::OriVertex};
use crate::graphs::{EIdx, Graph, VIdx, VecEdge, VecVertex};
use crate::indices::IdxCore;

pub struct GraphExtended<'a, G, Dv, De>
where
    G: Graph,
{
    pub(super) core: &'a G,
    pub(super) core_vertices: VecVertex<OriVertex<'a, G::V<'a>, Dv>>,
    pub(super) core_edges: VecEdge<OriEdge<'a, G::E<'a>, De>>,
    pub(super) new_vertices: VecVertex<VertexCore<Dv>>,
    pub(super) new_edges: VecEdge<EdgeCore<De>>,
}

impl<'a, G, Dv, De> GraphExtended<'a, G, Dv, De>
where
    G: Graph,
{
    pub(super) fn new_v_idx(&self, v: VIdx) -> Option<VIdx> {
        let v = v.into_inner();
        match v < self.core.v() {
            true => None,
            false => Some(VIdx::from(v - self.core.v())),
        }
    }

    pub(super) fn new_e_idx(&self, e: EIdx) -> Option<EIdx> {
        let e = e.into_inner();
        match e < self.core.e() {
            true => None,
            false => Some(EIdx::from(e - self.core.e())),
        }
    }
}
