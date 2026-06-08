use crate::graphs::{GraphBuilder, GraphCore, VIdx};
use crate::indices::IdxCore;
use crate::networks::core::connection::add_connection_edges;
use crate::networks::core::waiting::add_waiting_edges;
use crate::networks::core::{edge::CoreNwEdge, vertex::CoreNwVertex};
use crate::transports::Transport;
use crate::{CoreNw, Problem, Variant};

pub struct CoreNwBuilder<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) builder: GraphBuilder<CoreNwVertex, CoreNwEdge>,
}

impl<'a, V: Variant> CoreNwBuilder<'a, V> {
    pub fn initiate(p: &'a Problem<V>) -> Self {
        // vertices

        let rng = |len: usize| 0..len;

        let transports = rng(p.len_transports())
            .map(Transport::from)
            .map(CoreNwVertex::Transport);

        let builder = GraphCore::builder(transports);

        Self { p, builder }
    }

    pub fn finish(self) -> CoreNw<'a, V> {
        CoreNw::new(self.p, self.builder.finish())
    }

    pub fn transport_vidx(&self, t: Transport) -> VIdx {
        VIdx::from(t.into_inner())
    }

    pub fn split_graph(&mut self) -> (&Self, &mut GraphBuilder<CoreNwVertex, CoreNwEdge>) {
        let graph = unsafe { &mut *(&mut self.builder as *mut GraphBuilder<_, _>) };
        (self, graph)
    }
}

impl<V: Variant> Problem<V> {
    pub fn core_network(&self) -> CoreNw<'_, V> {
        let mut builder = CoreNwBuilder::initiate(self);

        let b = &mut builder;
        add_connection_edges(b);
        add_waiting_edges(b);

        builder.finish()
    }
}
