use crate::graphs::graph_extended::builder::ExtGraphBuilder;
use crate::graphs::graph_extended::edge::CoreEdge;
use crate::graphs::graph_extended::vertex::CoreVertex;
use crate::graphs::{Edge, GraphCore, VecEdge, VecVertex, Vertex};

pub struct ExtGraph<'a, V, E, Ve, Ee> {
    pub(super) core: &'a GraphCore<V, E>,
    pub(super) core_vertices: VecVertex<CoreVertex<V, E, Ve>>,
    pub(super) core_edges: VecEdge<CoreEdge<Ee>>,
    pub(super) ext_vertices: VecVertex<Vertex<Ve>>,
    pub(super) ext_edges: VecEdge<Edge<Ee>>,
}

impl<'a, V, E, Ve, Ee> ExtGraph<'a, V, E, Ve, Ee> {
    pub fn builder(
        core: &'a GraphCore<V, E>,
        core_vertices: impl Iterator<Item = Ve>,
        core_edges: impl Iterator<Item = Ee>,
        ext_vertices: impl Iterator<Item = Ve>,
    ) -> ExtGraphBuilder<'a, V, E, Ve, Ee> {
        ExtGraphBuilder::new(core, core_vertices, core_edges, ext_vertices)
    }

    pub fn len_vertices(&self) -> usize {
        self.core_vertices.len() + self.ext_vertices.len()
    }

    pub fn len_edges(&self) -> usize {
        self.core_edges.len() + self.ext_edges.len()
    }
}
