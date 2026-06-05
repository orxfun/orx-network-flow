use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::networks::aon::sinks::{SinkIdx, Sinks};
use crate::networks::aon::sources::{SourceIdx, Sources};
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};
use crate::space_time::SpaceTime;
use crate::transports::Transport;
use crate::{AonNetwork, Graph, Problem, Variant};

pub struct AonNetworkBuilder<'a, V: Variant> {
    p: &'a Problem<V>,
    builder: GraphBuilder<AonVertex, AonEdge>,
    sources: Sources,
    sinks: Sinks,
}

impl<'a, V: Variant> AonNetworkBuilder<'a, V> {
    pub fn initiate(p: &'a Problem<V>) -> Self {
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

        let builder = Graph::builder(vertices);

        Self {
            p,
            builder,
            sources,
            sinks,
        }
    }

    pub fn finish(self) -> AonNetwork<'a, V> {
        AonNetwork::new(self.p, self.builder.finish(), self.sources, self.sinks)
    }
}
