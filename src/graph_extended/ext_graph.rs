use crate::Graph;
use crate::graph::{VecEdge, VecVertex};

pub struct ExtGraph<V, E, V0, E0> {
    core: Graph<V0, E0>,
    ext_core_vertex_data: VecVertex<V>,
    ext_core_edge_data: VecEdge<E>,
}
