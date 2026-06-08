use crate::graphs::graph_extended::vertex::{ExtVertex, ExtVertexMut};
use crate::graphs::graph_extended::{edge::CoreEdge, ext_graph::ExtGraph, vertex::CoreVertex};
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, VecVertex, Vertex};
use crate::indices::IdxCore;

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
        assert_eq!(core_vertices.len(), core.len_vertices());

        let new_core_edge = |(i, data): (usize, Ee)| CoreEdge::new(EIdx::from(i), data);
        let core_edges: VecEdge<_> = core_edges.enumerate().map(new_core_edge).collect();
        assert_eq!(core_edges.len(), core.len_edges());

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

    pub fn edge(&mut self, data: Ee, tail: VIdx, head: VIdx) {
        let edges_idx = EIdx::from(self.0.len_edges());

        let tail_out_edge_idx = self.vertex(tail).len_out_edges();
        let head_in_edge_idx = self.vertex(head).len_in_edges();

        self.0.ext_edges.push(Edge::new(tail, head, data));

        self.vertex_mut(tail)
            .add_out_edge(edges_idx, head, head_in_edge_idx);
        self.vertex_mut(head)
            .add_in_edge(edges_idx, tail, tail_out_edge_idx);
    }

    pub fn finish(self) -> ExtGraph<'a, V, E, Ve, Ee> {
        self.0
    }

    // helpers

    fn vertex(&self, vidx: VIdx) -> ExtVertex<'_, V, E, Ve> {
        let idx = vidx.into_inner();
        match idx < self.0.core_vertices.len() {
            true => ExtVertex::Core(self.0.core, &self.0.core_vertices[vidx]),
            false => {
                let vidx = VIdx::from(idx - self.0.core_vertices.len());
                ExtVertex::Ext(&self.0.ext_vertices[vidx])
            }
        }
    }

    fn vertex_mut(&mut self, vidx: VIdx) -> ExtVertexMut<'_, V, E, Ve> {
        let idx = vidx.into_inner();
        match idx < self.0.core_vertices.len() {
            true => ExtVertexMut::Core(self.0.core, &mut self.0.core_vertices[vidx]),
            false => {
                let vidx = VIdx::from(idx - self.0.core_vertices.len());
                ExtVertexMut::Ext(&mut self.0.ext_vertices[vidx])
            }
        }
    }
}
