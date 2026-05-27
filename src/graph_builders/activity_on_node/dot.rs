use crate::Graph;
use crate::graph::{DotData, VIdx, Vertex};
use crate::graph_builders::activity_on_node::{EdgeData, VertexData};
use alloc::format;
use alloc::string::String;

impl DotData for (VertexData, EdgeData) {
    type V = VertexData;

    type E = EdgeData;

    fn vertex_label(_: &Graph<Self::V, Self::E>, v: VIdx, vertex: &Vertex<Self::V>) -> String {
        match vertex.data() {
            VertexData::Transport(t) => format!("{}\nm{}", v, t),
            VertexData::Source(c) => format!("{}\ns{}", v, c),
            VertexData::Sink(c) => format!("{}\nt{}", v, c),
        }
    }
}
