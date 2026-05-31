use crate::graph::{DotGraph, VIdx, Vertex};
use crate::graph_builders::activity_on_node::{EdgeData, VertexData};
use crate::{Graph, Problem, Variant};
use alloc::format;
use alloc::string::{String, ToString};

// impl DotData for (VertexData, EdgeData) {
//     type V = VertexData;

//     type E = EdgeData;

//     fn vertex_label(_: &Graph<Self::V, Self::E>, v: VIdx, vertex: &Vertex<Self::V>) -> String {
//         match vertex.data() {
//             VertexData::Transport(t) => {
//                 // TODO: abc

//                 format!("{}\nm{}", v, t)
//             }
//             VertexData::Source(c) => format!("{}\ns{}", v, c),
//             VertexData::Sink(c) => format!("{}\nt{}", v, c),
//         }
//     }
// }

pub struct AonDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    graph: &'a Graph<VertexData, EdgeData>,
}

impl<'a, V: Variant> AonDotGraph<'a, V> {
    pub fn new(problem: &'a Problem<V>, graph: &'a Graph<VertexData, EdgeData>) -> Self {
        Self { problem, graph }
    }
}

impl<'a, V: Variant> DotGraph for AonDotGraph<'a, V> {
    type V = VertexData;

    type E = EdgeData;

    fn graph(&self) -> &Graph<Self::V, Self::E> {
        self.graph
    }

    fn vertex_label(&self, v: VIdx, vertex: &Vertex<Self::V>) -> String {
        let prob = self.problem;
        match vertex.data() {
            VertexData::Source(c) => {
                let commodity = prob.commodity_by_idx(*c);
                let ori = prob.space_key(commodity.origin().space());
                let des = prob.space_key(commodity.destination().space());
                let rt = commodity.origin().time();
                format!("{} : s{}\n{}-{}\tready: {}", v, c, ori, des, rt)
            }
            VertexData::Sink(c) => {
                let commodity = prob.commodity_by_idx(*c);
                let ori = prob.space_key(commodity.origin().space());
                let des = prob.space_key(commodity.destination().space());
                let due = commodity.destination().time();
                format!("{} : t{}\n{}-{}\tdue: {}", v, c, ori, des, due)
            }
            VertexData::Transport(t) => {
                let transport = prob.transport_by_idx(*t);
                let ori = prob.space_key(transport.origin().space());
                let des = prob.space_key(transport.destination().space());
                let dt = transport.origin().time();
                let at = transport.destination().time();
                format!("{}\n{}-{}\t{}-{}", v, ori, des, dt, at)
            }
        }
    }
}
