use crate::graphs::{Edge, Graph, VecVertex, Vertex, extended::vertex::OriVertex};

pub struct GraphExtended<'a, G, Dv, De>
where
    G: Graph,
{
    pub(super) core: &'a G,
    pub(super) core_vertices: VecVertex<OriVertex<'a, G::V<'a>, Dv>>,

    // pub(super) core_edges: VecVertex<OriVertex<V>>,

    // abc
    x: (Dv, De), // pub(super) core_vertices: VecVertex<CoreVertex<V, E, Ve>>,
                 // pub(super) core_edges: VecEdge<CoreEdge<Ee>>,
                 // pub(super) ext_vertices: VecVertex<Vertex<Ve>>,
                 // pub(super) ext_edges: VecEdge<Edge<Ee>>,
}
