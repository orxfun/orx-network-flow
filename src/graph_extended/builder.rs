use crate::graph::{Edge, OutEdge, VecEdge, VecVertex, Vertex};
use crate::graph_extended::{edge::CoreEdge, ext_graph::ExtGraph, vertex::CoreVertex};
use crate::indices::IdxCore;
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

    pub fn edge(&mut self, data: Ee, tail: VIdx, head: VIdx) {
        let edges_idx = EIdx::from(self.0.len_edges());

        match tail.into_inner() < self.0.core_vertices.len() {
            true => {
                let vertex = &self.0.core_vertices[tail];
                let core_out_edges = self.0.core.vertex(vertex.core_vidx).out_edges().len();
                let new_out_edges = vertex.ext_out_edges.len();
                let tail_out_edge_idx = core_out_edges + new_out_edges;
            }
            false => {
                let vidx = VIdx::from(tail.into_inner() - self.0.core_vertices.len());
                let vertex = &self.0.ext_vertices[vidx];
                let tail_out_edge_idx = vertex.out_edges().len();
            }
        }

        match head.into_inner() < self.0.core_vertices.len() {
            true => {
                let vertex = &self.0.core_vertices[head];
                let core_in_edges = self.0.core.vertex(vertex.core_vidx).in_edges().len();
                let new_in_edges = vertex.ext_in_edges.len();
                let head_in_edge_idx = core_in_edges + new_in_edges;
            }
            false => {
                let vidx = VIdx::from(head.into_inner() - self.0.core_vertices.len());
                let vertex = &self.0.ext_vertices[vidx];
                let head_in_edge_idx = vertex.in_edges().len();
            }
        }

        // let tail_out_edge_idx = self.0.vertex(tail).len_out_edges();
        // let head_in_edge_idx = self.0.vertex(head).len_in_edges();
        // self.0.ext_edges.push(Edge::new(tail, head, data));
        // self.0
        //     .vertex(tail)
        //     .add_out_edge(edges_idx, head, head_in_edge_idx);
        // self.0
        //     .vertex(head)
        //     .add_in_edge(edges_idx, tail, tail_out_edge_idx);
    }

    pub(super) fn add_out_edge(
        &mut self,
        vidx: VIdx,
        edges_idx: EIdx,
        head: VIdx,
        head_in_edge_idx: usize,
    ) {
        //
    }
}
