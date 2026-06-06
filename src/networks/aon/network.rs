use crate::networks::aon::sinks::{Sink, SinkIdx, Sinks};
use crate::networks::aon::sources::{Source, SourceIdx, Sources};
use crate::networks::aon::visualization::dot::{AonDotGraph, AonDotGraphSettings};
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};
use crate::space_time::SpaceTime;
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
        self.sources.get_st(idx).expect("invalid source idx")
    }

    pub fn source(&self, idx: SourceIdx) -> &Source {
        self.sources.get_by_idx(idx).expect("invalid source idx")
    }

    pub fn sink_st(&self, idx: SinkIdx) -> SpaceTime {
        self.sinks.get_st(idx).expect("invalid sink idx")
    }

    pub fn sink(&self, idx: SinkIdx) -> &Sink {
        self.sinks.get_by_idx(idx).expect("invalid sink idx")
    }

    // visualization

    pub fn dot(&'a self, custom_settings: Option<AonDotGraphSettings>) -> AonDotGraph<'a, V> {
        match custom_settings {
            Some(settings) => AonDotGraph::with_settings(self.p, self, settings),
            None => AonDotGraph::new(self.p, self),
        }
    }
}
