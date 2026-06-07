use crate::graph::{GraphBuilder, VIdx};
use crate::indices::IdxCore;
use crate::networks::core::connection::add_connection_edges;
use crate::networks::core::waiting::add_waiting_edges;
use crate::networks::core::{edge::AonEdge, vertex::AonVertex};
use crate::transports::Transport;
use crate::{CoreNetwork, Graph, Problem, Variant};

pub struct AonNetworkBuilder<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) builder: GraphBuilder<AonVertex, AonEdge>,
}

impl<'a, V: Variant> AonNetworkBuilder<'a, V> {
    pub fn initiate(p: &'a Problem<V>) -> Self {
        // vertices

        let rng = |len: usize| 0..len;

        let transports = rng(p.len_transports())
            .map(Transport::from)
            .map(AonVertex::Transport);

        let builder = Graph::builder(transports);

        Self { p, builder }
    }

    pub fn finish(self) -> CoreNetwork<'a, V> {
        CoreNetwork::new(self.p, self.builder.finish())
    }

    pub fn transport_vidx(&self, t: Transport) -> VIdx {
        VIdx::from(t.into_inner())
    }

    pub fn split_graph(&mut self) -> (&Self, &mut GraphBuilder<AonVertex, AonEdge>) {
        let graph = unsafe { &mut *(&mut self.builder as *mut GraphBuilder<_, _>) };
        (self, graph)
    }
}

impl<V: Variant> Problem<V> {
    pub fn core_network(&self) -> CoreNetwork<'_, V> {
        let mut builder = AonNetworkBuilder::initiate(self);

        let b = &mut builder;
        add_connection_edges(b);
        add_waiting_edges(b);

        builder.finish()
    }
}
