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

    pub fn edge(&mut self, data: E, tail: VIdx, head: VIdx) {
        let edges_idx = EIdx::from(self.0.len_edges());

        // let tail_out_edge_idx = self.0.vertices[tail].out_edges().len();
        // let head_in_edge_idx = self.0.vertices[head].in_edges().len();
        // self.0.edges.push(Edge::new(tail, head, data));
        // self.0.vertices[tail].add_out_edge(edges_idx, head, head_in_edge_idx);
        // self.0.vertices[head].add_in_edge(edges_idx, tail, tail_out_edge_idx);
    }
}
