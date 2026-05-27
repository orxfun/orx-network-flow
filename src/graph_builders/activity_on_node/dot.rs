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
            VertexData::Transport(t) => format!("m{}\n{}", t, v),
            VertexData::Source(c) => format!("s{}\n{}", c, v),
            VertexData::Sink(c) => format!("t{}\n{}", c, v),
        }
    }
}
