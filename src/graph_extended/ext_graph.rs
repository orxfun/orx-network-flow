use crate::graph::{Edge, VecEdge, VecVertex, Vertex};
use crate::graph_extended::builder::ExtGraphBuilder;
use crate::graph_extended::edge::CoreEdge;
use crate::graph_extended::vertex::{CoreVertex, ExtVertex};
use crate::indices::IdxCore;
use crate::{Graph, VIdx};

pub struct ExtGraph<'a, V, E, Ve, Ee> {
    pub(super) core: &'a Graph<V, E>,
    pub(super) core_vertices: VecVertex<CoreVertex<V, E, Ve>>,
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
        ExtGraphBuilder::new(core, core_vertices, core_edges, ext_vertices)
    }

    pub fn len_vertices(&self) -> usize {
        self.core_vertices.len() + self.ext_vertices.len()
    }

    pub fn len_edges(&self) -> usize {
        self.core_edges.len() + self.ext_edges.len()
    }

    // helpers

    pub(super) fn vertex(&self, vidx: VIdx) -> ExtVertex<'_, V, E, Ve> {
        let idx = vidx.into_inner();
        match idx < self.core_vertices.len() {
            true => ExtVertex::Core(&self.core_vertices[vidx]),
            false => {
                let vidx = VIdx::from(idx - self.core_vertices.len());
                ExtVertex::Ext(&self.ext_vertices[vidx])
            }
        }
    }
}
