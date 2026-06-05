use crate::commodities::Commodity;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use crate::networks::aon::sinks::{SinkIdx, Sinks};
use crate::networks::aon::source_to_source::add_source_to_source_edges;
use crate::networks::aon::sources::{SourceIdx, Sources};
use crate::networks::aon::visualization::dot::{AonDotGraph, AonDotGraphSettings};
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};
use crate::space_time::SpaceTime;
use crate::transports::Transport;
use crate::{Graph, Problem, Variant};

#[derive(derive_new::new)]
pub struct AonNetwork<'a, V: Variant> {
    p: &'a Problem<V>,
    graph: Graph<AonVertex, AonEdge>,
    sources: Sources,
    sinks: Sinks,
}

impl<'a, V: Variant> AonNetwork<'a, V> {
    pub fn graph(&self) -> &Graph<AonVertex, AonEdge> {
        &self.graph
    }

    pub fn source_st(&self, idx: SourceIdx) -> SpaceTime {
        // self.sources.get_st(idx).expect("invalid source idx")
        todo!()
    }

    pub fn sink_st(&self, idx: SinkIdx) -> SpaceTime {
        self.sinks.get_st(idx).expect("invalid sink idx")
    }

    // visualization

    pub fn dot(&'a self, custom_settings: Option<AonDotGraphSettings>) -> AonDotGraph<'a, V> {
        match custom_settings {
            Some(settings) => AonDotGraph::with_settings(self.p, self, settings),
            None => AonDotGraph::new(self.p, self),
        }
    }
}
