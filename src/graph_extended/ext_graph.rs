use crate::Graph;
use crate::graph::{Edge, VecEdge, VecVertex, Vertex};
use crate::graph_extended::builder::ExtGraphBuilder;
use crate::graph_extended::edge::CoreEdge;
use crate::graph_extended::vertex::CoreVertex;

pub struct ExtGraph<'a, V, E, Ve, Ee> {
    pub(super) core: &'a Graph<V, E>,
    pub(super) core_vertices: VecVertex<CoreVertex<Ve>>,
    pub(super) core_edges: VecEdge<CoreEdge<Ee>>,
    pub(super) ext_vertices: VecVertex<Vertex<Ve>>,
    pub(super) ext_edges: VecEdge<Edge<Ee>>,
}

impl<'a, V, E, Ve, Ee> ExtGraph<'a, V, E, Ve, Ee> {
    pub fn builder(
        core: &'a Graph<V, E>,
        core_vertices: impl Iterator<Item = Ve>,
        core_edges: impl Iterator<Item = Ee>,
        ext_vertices: impl Iterator<Item = Ve>,
    ) -> ExtGraphBuilder<'a, V, E, Ve, Ee> {
        // GraphBuilder::new(vertices)
        todo!()
    }
}
