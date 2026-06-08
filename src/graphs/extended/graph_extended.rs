use crate::graphs::{Graph, VecVertex};

pub struct GraphExtended<'a, G, V, E>
where
    G: Graph,
{
    pub(super) core: &'a G,
    x: (V, E), // pub(super) core_vertices: VecVertex<CoreVertex<V, E, Ve>>,
               // pub(super) core_edges: VecEdge<CoreEdge<Ee>>,
               // pub(super) ext_vertices: VecVertex<Vertex<Ve>>,
               // pub(super) ext_edges: VecEdge<Edge<Ee>>,
}
