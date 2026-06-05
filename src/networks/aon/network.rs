use crate::commodities::Commodity;
use crate::networks::aon::sinks::{SinkIdx, Sinks};
use crate::networks::aon::sources::{SourceIdx, Sources};
use crate::networks::aon::visualization::dot::{AonDotGraph, AonDotGraphSettings};
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};
use crate::space_time::SpaceTime;
use crate::transports::Transport;
use crate::{Graph, Problem, Variant};

pub struct AonNetwork<'a, V: Variant> {
    p: &'a Problem<V>,
    graph: Graph<AonVertex, AonEdge>,
    sources: Sources,
    sinks: Sinks,
}

impl<'a, V: Variant> AonNetwork<'a, V> {
    pub fn create(p: &'a Problem<V>) -> Self {
        // vertices

        let rng = |len: usize| 0..len;

        let transports = rng(p.len_transports())
            .map(Transport::from)
            .map(AonVertex::Transport);

        let sources = Sources::create(p);
        let source_vertices = rng(sources.len())
            .map(SourceIdx::from)
            .map(AonVertex::Source);

        let sinks = Sinks::create(p);
        let sink_vertices = rng(sinks.len()).map(SinkIdx::from).map(AonVertex::Sink);

        let teleports = rng(p.len_commodities())
            .map(Commodity::from)
            .map(AonVertex::Teleport);

        let vertices = transports
            .chain(teleports)
            .chain(source_vertices)
            .chain(sink_vertices);

        let mut builder = Graph::builder(vertices);

        // edges

        // finalize

        let graph = builder.finish();

        Self {
            p,
            graph,
            sources,
            sinks,
        }
    }

    pub fn graph(&self) -> &Graph<AonVertex, AonEdge> {
        &self.graph
    }

    pub fn source_st(&self, idx: SourceIdx) -> SpaceTime {
        self.sources.get_st(idx).expect("invalid source idx")
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
