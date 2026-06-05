use crate::commodities::Commodity;
use crate::graph::{GraphBuilder, VIdx};
use crate::indices::IdxCore;
use crate::networks::aon::sinks::{SinkIdx, Sinks};
use crate::networks::aon::source_to_source::add_source_to_source_edges;
use crate::networks::aon::sources::{SourceIdx, Sources};
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};
use crate::space_time::SpaceTime;
use crate::transports::Transport;
use crate::{AonNetwork, Graph, Problem, Variant};

pub struct AonNetworkBuilder<'a, V: Variant> {
    p: &'a Problem<V>,
    pub(super) builder: GraphBuilder<AonVertex, AonEdge>,
    pub(super) sources: Sources,
    pub(super) sinks: Sinks,
    offset_sources: usize,
    offset_sinks: usize,
    offset_teleports: usize,
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

        let offset_teleports = p.len_transports();
        let offset_sources = offset_teleports + p.len_commodities();
        let offset_sinks = offset_sources + sources.len();

        Self {
            p,
            builder,
            sources,
            sinks,
            offset_sources,
            offset_sinks,
            offset_teleports,
        }
    }

    pub fn finish(self) -> AonNetwork<'a, V> {
        AonNetwork::new(self.p, self.builder.finish(), self.sources, self.sinks)
    }

    fn s_to_vidx(&self, s: SourceIdx) -> VIdx {
        VIdx::from(self.offset_sources + s.into_inner())
    }

    fn t_to_vidx(&self, t: SinkIdx) -> VIdx {
        VIdx::from(self.offset_sinks + t.into_inner())
    }

    pub fn source_vidx(&self, st: SpaceTime) -> VIdx {
        let s = self.sources.get_s_idx(st).expect("invalid source st");
        self.s_to_vidx(s)
    }

    pub fn sink_vidx(&self, st: SpaceTime) -> VIdx {
        let t = self.sinks.get_t_idx(st).expect("invalid sink st");
        self.t_to_vidx(t)
    }
}

impl<V: Variant> Problem<V> {
    pub fn aon_network(&self) -> AonNetwork<'_, V> {
        let mut builder = AonNetworkBuilder::initiate(self);

        add_source_to_source_edges(&mut builder);

        builder.finish()
    }
}
