use std::string::ToString;

use crate::graph::visualization::dot::DotGraph;
use crate::graph::{VIdx, Vertex};
use crate::graph_builders::activity_on_node::visualization::dot::AonDotGraphSettings;
use crate::graph_builders::activity_on_node::{EdgeData, VertexData};
use crate::{Graph, Problem, Variant};
use alloc::format;
use alloc::string::String;

pub struct AonDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    graph: &'a Graph<VertexData, EdgeData>,
    settings: AonDotGraphSettings,
}

impl<'a, V: Variant> AonDotGraph<'a, V> {
    pub fn new(problem: &'a Problem<V>, graph: &'a Graph<VertexData, EdgeData>) -> Self {
        Self::with_settings(problem, graph, Default::default())
    }

    pub fn with_settings(
        problem: &'a Problem<V>,
        graph: &'a Graph<VertexData, EdgeData>,
        settings: AonDotGraphSettings,
    ) -> Self {
        Self {
            problem,
            graph,
            settings,
        }
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

    fn vertex_settings(&self, _: VIdx, vertex: &Vertex<Self::V>) -> String {
        match vertex.data() {
            VertexData::Source(_) => self.settings.source.to_string(),
            VertexData::Sink(_) => self.settings.sink.to_string(),
            VertexData::Transport(_) => self.settings.transport.to_string(),
        }
    }
}
