use crate::graphs::graph_core::{EdgeCore, VertexCore};
use crate::graphs::{EIdx, Edge, Graph, GraphCore, VIdx, VecEdge, VecVertex, Vertex};

pub struct GraphBuilder<V, E>(GraphCore<V, E>);

impl<V, E> GraphBuilder<V, E> {
    pub fn new(vertices: impl Iterator<Item = V>) -> Self {
        let vertices: VecVertex<_> = vertices.map(VertexCore::new).collect();
        let edges = VecEdge::new();
        let graph = GraphCore { vertices, edges };
        Self(graph)
    }

    pub fn edge(&mut self, data: E, tail: VIdx, head: VIdx) {
        let edges_idx = EIdx::from(self.0.edges.len());
        let tail_out_edge_idx = self.0.vertex(tail).out_edges().len();
        let head_in_edge_idx = self.0.vertex(head).in_edges().len();
        self.0.edges.push(EdgeCore::new(tail, head, data));
        self.0.vertices[tail].add_out_edge(edges_idx, head, head_in_edge_idx);
        self.0.vertices[head].add_in_edge(edges_idx, tail, tail_out_edge_idx);
    }

    pub fn validate(&self) {
        let abc = 12;
        // let num_nodes = VIdx::from(self.0.vertices.len());
        // let num_edges = EIdx::from(self.0.edges.len());

        // // Ensure all node-side references are valid and consistent with edge endpoints.
        // for (tail_idx, node) in self
        //     .0
        //     .vertices
        //     .iter()
        //     .enumerate()
        //     .map(|(a, b)| (VIdx::from(a), b))
        // {
        //     for (tail_out_edge_idx, edge_idx) in node.out_edges().enumerate() {
        //         assert!(
        //             edge_idx < num_edges,
        //             "out edge has invalid edge index: node={tail_idx}, out_edge_idx={tail_out_edge_idx}, edge_idx={edge_idx:?}, num_edges={num_edges}"
        //         );

        //         let edge = &self.0.edges[edge_idx];
        //         let head_idx = out_edge.head();
        //         assert!(
        //             head_idx < num_nodes,
        //             "out edge has invalid head node index: node={tail_idx}, out_edge_idx={tail_out_edge_idx}, head={head_idx}, num_nodes={num_nodes}"
        //         );

        //         assert_eq!(
        //             edge.tail(),
        //             tail_idx,
        //             "edge tail mismatch for out edge: node={tail_idx}, out_edge_idx={tail_out_edge_idx}, edge_idx={edge_idx}"
        //         );
        //         assert_eq!(
        //             edge.head(),
        //             head_idx,
        //             "edge head mismatch for out edge: node={tail_idx}, out_edge_idx={tail_out_edge_idx}, edge_idx={edge_idx}"
        //         );

        //         let head_in_edge_idx = out_edge.head_in_edge_idx();
        //         let head_node = &self.0.vertices[head_idx];
        //         assert!(
        //             head_in_edge_idx < head_node.in_edges().len(),
        //             "out edge points to missing reciprocal in edge: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, head_in_edge_idx={head_in_edge_idx}"
        //         );

        //         let reciprocal_in = &head_node.in_edges()[head_in_edge_idx];
        //         assert_eq!(
        //             reciprocal_in.edges_idx(),
        //             edge_idx,
        //             "reciprocal in edge edge_idx mismatch: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, head_in_edge_idx={head_in_edge_idx}"
        //         );
        //         assert_eq!(
        //             reciprocal_in.tail(),
        //             tail_idx,
        //             "reciprocal in edge tail mismatch: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, head_in_edge_idx={head_in_edge_idx}"
        //         );
        //         assert_eq!(
        //             reciprocal_in.tail_out_edge_idx(),
        //             tail_out_edge_idx,
        //             "reciprocal in edge tail_out_edge_idx mismatch: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, head_in_edge_idx={head_in_edge_idx}"
        //         );
        //     }
        // }

        // // Ensure all in-edge references have a valid reciprocal out-edge.
        // for (head_idx, node) in self
        //     .0
        //     .vertices
        //     .iter()
        //     .enumerate()
        //     .map(|(a, b)| (VIdx::from(a), b))
        // {
        //     for (head_in_edge_idx, in_edge) in node.in_edges().iter().enumerate() {
        //         let edge_idx = in_edge.edges_idx();
        //         assert!(
        //             edge_idx < num_edges,
        //             "in edge has invalid edge index: node={head_idx}, in_edge_idx={head_in_edge_idx}, edge_idx={edge_idx}, num_edges={num_edges}"
        //         );

        //         let edge = &self.0.edges[edge_idx];
        //         let tail_idx = in_edge.tail();
        //         assert!(
        //             tail_idx < num_nodes,
        //             "in edge has invalid tail node index: node={head_idx}, in_edge_idx={head_in_edge_idx}, tail={tail_idx}, num_nodes={num_nodes}"
        //         );

        //         assert_eq!(
        //             edge.tail(),
        //             tail_idx,
        //             "edge tail mismatch for in edge: node={head_idx}, in_edge_idx={head_in_edge_idx}, edge_idx={edge_idx}"
        //         );
        //         assert_eq!(
        //             edge.head(),
        //             head_idx,
        //             "edge head mismatch for in edge: node={head_idx}, in_edge_idx={head_in_edge_idx}, edge_idx={edge_idx}"
        //         );

        //         let tail_out_edge_idx = in_edge.tail_out_edge_idx();
        //         let tail_node = &self.0.vertices[tail_idx];
        //         assert!(
        //             tail_out_edge_idx < tail_node.out_edges().len(),
        //             "in edge points to missing reciprocal out edge: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, tail_out_edge_idx={tail_out_edge_idx}"
        //         );

        //         let reciprocal_out = &tail_node.out_edges()[tail_out_edge_idx];
        //         assert_eq!(
        //             reciprocal_out.edges_idx(),
        //             edge_idx,
        //             "reciprocal out edge edge_idx mismatch: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, tail_out_edge_idx={tail_out_edge_idx}"
        //         );
        //         assert_eq!(
        //             reciprocal_out.head(),
        //             head_idx,
        //             "reciprocal out edge head mismatch: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, tail_out_edge_idx={tail_out_edge_idx}"
        //         );
        //         assert_eq!(
        //             reciprocal_out.head_in_edge_idx(),
        //             head_in_edge_idx,
        //             "reciprocal out edge head_in_edge_idx mismatch: tail={tail_idx}, head={head_idx}, edge_idx={edge_idx}, tail_out_edge_idx={tail_out_edge_idx}"
        //         );
        //     }
        // }
    }

    pub fn finish(self) -> GraphCore<V, E> {
        self.0
    }
}
