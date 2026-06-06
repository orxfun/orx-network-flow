use crate::networks::core::sinks::{SinkIdx, Sinks};
use crate::networks::core::sources::{SourceIdx, Sources};
use crate::networks::core::visualization::dot::{AonDotGraphSettings, CoreDotGraph};
use crate::networks::core::{edge::AonEdge, vertex::AonVertex};
use crate::space_time::SpaceTime;
use crate::time::Time;
use crate::{Graph, Problem, Variant};

#[derive(derive_new::new)]
pub struct CoreNetwork<'a, V: Variant> {
    p: &'a Problem<V>,
    graph: Graph<AonVertex, AonEdge>,
    sources: Sources,
    sinks: Sinks,
}

impl<'a, V: Variant> CoreNetwork<'a, V> {
    pub fn graph(&self) -> &Graph<AonVertex, AonEdge> {
        &self.graph
    }

    pub fn source_st(&self, idx: SourceIdx) -> SpaceTime {
        self.sources.get_st(idx).expect("invalid source idx")
    }

    pub fn source(&self, idx: SourceIdx) -> Time {
        self.sources.get_by_idx(idx).expect("invalid source idx")
    }

    pub fn sink_st(&self, idx: SinkIdx) -> SpaceTime {
        self.sinks.get_st(idx).expect("invalid sink idx")
    }

    pub fn sink(&self, idx: SinkIdx) -> Time {
        self.sinks.get_by_idx(idx).expect("invalid sink idx")
    }

    // visualization

    pub fn dot(&'a self, custom_settings: Option<AonDotGraphSettings>) -> CoreDotGraph<'a, V> {
        match custom_settings {
            Some(settings) => CoreDotGraph::with_settings(self.p, self, settings),
            None => CoreDotGraph::new(self.p, self),
        }
    }
}
