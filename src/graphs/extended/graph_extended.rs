use crate::graphs::{Graph, VecVertex, extended::vertex::OriVertex};

pub struct GraphExtended<'a, G, Vc, Ec, V, E>
where
    G: Graph<V = Vc, E = Ec>,
{
    pub(super) core: &'a G,
    // pub(super) core_vertices: VecVertex<OriVertex<V>>,
    // pub(super) core_edges: VecVertex<OriVertex<V>>,

    // abc
    x: (V, E), // pub(super) core_vertices: VecVertex<CoreVertex<V, E, Ve>>,
               // pub(super) core_edges: VecEdge<CoreEdge<Ee>>,
               // pub(super) ext_vertices: VecVertex<Vertex<Ve>>,
               // pub(super) ext_edges: VecEdge<Edge<Ee>>,
}
