use crate::graph::{VecEdge, VecVertex, Vertex};
use crate::graph_extended::{edge::CoreEdge, ext_graph::ExtGraph, vertex::CoreVertex};
use crate::{EIdx, Graph, VIdx};

pub struct ExtGraphBuilder<'a, V, E, Ve, Ee>(ExtGraph<'a, V, E, Ve, Ee>);

impl<'a, V, E, Ve, Ee> ExtGraphBuilder<'a, V, E, Ve, Ee> {
    pub fn new(
        core: &'a Graph<V, E>,
        core_vertices: impl Iterator<Item = Ve>,
        core_edges: impl Iterator<Item = Ee>,
        ext_vertices: impl Iterator<Item = Ve>,
    ) -> Self {
        let new_core_vertex = |(i, data): (usize, Ve)| CoreVertex::new(VIdx::from(i), data);
        let core_vertices: VecVertex<_> = core_vertices.enumerate().map(new_core_vertex).collect();

        let new_core_edge = |(i, data): (usize, Ee)| CoreEdge::new(EIdx::from(i), data);
        let core_edges: VecEdge<_> = core_edges.enumerate().map(new_core_edge).collect();

        let ext_vertices: VecVertex<_> = ext_vertices.map(Vertex::new).collect();
        let ext_edges = VecEdge::new();

        let graph = ExtGraph {
            core,
            core_vertices,
            core_edges,
            ext_vertices,
            ext_edges,
        };

        Self(graph)
    }
}
