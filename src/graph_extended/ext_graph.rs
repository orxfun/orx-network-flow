use crate::Graph;
use crate::graph::{Edge, VecEdge, VecVertex, Vertex};
use crate::graph_extended::edge::CoreEdge;
use crate::graph_extended::vertex::CoreVertex;

pub struct ExtGraph<'a, V, E, Ve, Ee> {
    core: &'a Graph<V, E>,
    core_vertices: VecVertex<CoreVertex<Ve>>,
    core_edges: VecEdge<CoreEdge<Ee>>,
    ext_vertices: VecVertex<Vertex<Ve>>,
    ext_edges: VecEdge<Edge<Ee>>,
}
