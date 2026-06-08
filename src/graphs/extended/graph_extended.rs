use crate::graphs::core::{EdgeCore, VertexCore};
use crate::graphs::extended::{edge::OriEdge, vertex::OriVertex};
use crate::graphs::{Graph, VecEdge, VecVertex};

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
