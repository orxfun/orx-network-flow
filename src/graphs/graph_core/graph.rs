use crate::graphs::{Edge, GraphBuilder, VIdx, VecEdge, VecVertex, Vertex, graph::Graph};

pub struct GraphCore<V, E> {
    pub(super) vertices: VecVertex<Vertex<V>>,
    pub(super) edges: VecEdge<Edge<E>>,
}

impl<V, E> GraphCore<V, E> {
    pub fn builder(vertices: impl Iterator<Item = V>) -> GraphBuilder<V, E> {
        GraphBuilder::new(vertices)
    }

    pub fn len_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn len_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn vertex(&self, vidx: VIdx) -> &Vertex<V> {
        &self.vertices[vidx]
    }
}

// impl<V, E> Graph for GraphCore<V, E> {
//     type V = V;

//     type E = E;

//     fn vertex(&self, v: crate::graphs::vertex::VIdx) -> &Vertex<Self::V> {
//         todo!()
//     }

//     fn edge(&self, e: crate::graphs::edge::EIdx) -> &Edge<Self::E> {
//         todo!()
//     }
// }
