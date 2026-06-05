use crate::networks::aon::indexer::Indexer;
use crate::networks::aon::sinks::{SinkIdx, Sinks};
use crate::networks::aon::sources::{SourceIdx, Sources};
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};
use crate::transports::Transport;
use crate::{Graph, Problem, Variant};

pub struct AonNetwork {
    graph: Graph<AonVertex, AonEdge>,
    indexer: Indexer,
}

impl AonNetwork {
    pub fn create<V: Variant>(p: &Problem<V>) -> Self {
        let sources = Sources::create(p);
        let sinks = Sinks::create(p);
        let num_transports = p.len_transports();
        let indexer = Indexer::new(num_transports, sources, sinks);

        // vertices

        let rng = |len: usize| 0..len;

        let transports = rng(p.len_transports())
            .map(Transport::from)
            .map(AonVertex::Transport);

        let sources = rng(indexer.len_sources())
            .map(SourceIdx::from)
            .map(AonVertex::Source);

        let sinks = rng(indexer.len_sinks())
            .map(SinkIdx::from)
            .map(AonVertex::Sink);

        let vertices = transports.chain(sources).chain(sinks);

        let mut builder = Graph::builder(vertices);

        // edges

        // finalize

        let graph = builder.finish();

        Self { graph, indexer }
    }

    pub fn graph(&self) -> &Graph<AonVertex, AonEdge> {
        &self.graph
    }
}
