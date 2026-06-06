use crate::graph::{GraphBuilder, VIdx};
use crate::indices::IdxCore;
use crate::networks::core::connection::add_connection_edges;
use crate::networks::core::sinks::{SinkIdx, Sinks};
use crate::networks::core::source_to_transport::add_source_to_transport_edges;
use crate::networks::core::sources::{SourceIdx, Sources};
use crate::networks::core::transport_to_sink::add_transport_to_sink_edges;
use crate::networks::core::waiting::add_waiting_edges;
use crate::networks::core::{edge::AonEdge, vertex::AonVertex};
use crate::space_time::SpaceTime;
use crate::transports::Transport;
use crate::{CoreNetwork, Graph, Problem, Variant};

pub struct AonNetworkBuilder<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) builder: GraphBuilder<AonVertex, AonEdge>,
    pub(super) sources: Sources,
    pub(super) sinks: Sinks,
    offset_sources: usize,
    offset_sinks: usize,
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

        let vertices = transports.chain(source_vertices).chain(sink_vertices);

        let builder = Graph::builder(vertices);

        let offset_sources = p.len_transports();
        let offset_sinks = offset_sources + sources.len();

        Self {
            p,
            builder,
            sources,
            sinks,
            offset_sources,
            offset_sinks,
        }
    }

    pub fn finish(self) -> CoreNetwork<'a, V> {
        CoreNetwork::new(self.p, self.builder.finish(), self.sources, self.sinks)
    }

    pub fn sidx_to_vidx(&self, sidx: SourceIdx) -> VIdx {
        VIdx::from(self.offset_sources + sidx.into_inner())
    }

    pub fn tidx_to_vidx(&self, tidx: SinkIdx) -> VIdx {
        VIdx::from(self.offset_sinks + tidx.into_inner())
    }

    pub fn transport_vidx(&self, t: Transport) -> VIdx {
        VIdx::from(t.into_inner())
    }

    pub fn source_vidx(&self, st: SpaceTime) -> VIdx {
        let s = self.sources.get_s_idx(st).expect("invalid source st");
        self.sidx_to_vidx(s)
    }

    pub fn sink_vidx(&self, st: SpaceTime) -> VIdx {
        let t = self.sinks.get_t_idx(st).expect("invalid sink st");
        self.tidx_to_vidx(t)
    }

    pub fn split_graph(&mut self) -> (&Self, &mut GraphBuilder<AonVertex, AonEdge>) {
        let graph = unsafe { &mut *(&mut self.builder as *mut GraphBuilder<_, _>) };
        (self, graph)
    }
}

impl<V: Variant> Problem<V> {
    pub fn aon_network(&self) -> CoreNetwork<'_, V> {
        let mut builder = AonNetworkBuilder::initiate(self);

        let b = &mut builder;
        add_source_to_transport_edges(b);
        add_transport_to_sink_edges(b);
        add_connection_edges(b);
        add_waiting_edges(b);

        builder.finish()
    }
}
