use crate::graph::{GraphBuilder, VIdx};
use crate::indices::IdxCore;
use crate::networks::transport_nw::connection::add_connection_edges;
use crate::networks::transport_nw::waiting::add_waiting_edges;
use crate::networks::transport_nw::{edge::TrNwEdge, vertex::TrNwVertex};
use crate::transports::Transport;
use crate::{Graph, Problem, TransportNw, Variant};

pub struct TransportNwBuilder<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) builder: GraphBuilder<TrNwVertex, TrNwEdge>,
}

impl<'a, V: Variant> TransportNwBuilder<'a, V> {
    pub fn initiate(p: &'a Problem<V>) -> Self {
        // vertices

        let rng = |len: usize| 0..len;

        let transports = rng(p.len_transports())
            .map(Transport::from)
            .map(TrNwVertex::Transport);

        let builder = Graph::builder(transports);

        Self { p, builder }
    }

    pub fn finish(self) -> TransportNw<'a, V> {
        TransportNw::new(self.p, self.builder.finish())
    }

    pub fn transport_vidx(&self, t: Transport) -> VIdx {
        VIdx::from(t.into_inner())
    }

    pub fn split_graph(&mut self) -> (&Self, &mut GraphBuilder<TrNwVertex, TrNwEdge>) {
        let graph = unsafe { &mut *(&mut self.builder as *mut GraphBuilder<_, _>) };
        (self, graph)
    }
}

impl<V: Variant> Problem<V> {
    pub fn core_network(&self) -> TransportNw<'_, V> {
        let mut builder = TransportNwBuilder::initiate(self);

        let b = &mut builder;
        add_connection_edges(b);
        add_waiting_edges(b);

        builder.finish()
    }
}
