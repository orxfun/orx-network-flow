use crate::graph::{InEdge, OutEdge, Vertex};
use crate::{Graph, VIdx};
use alloc::vec::Vec;
use core::marker::PhantomData;

pub enum ExtVertex<'a, V, E, Ve> {
    Core(&'a Graph<V, E>, &'a CoreVertex<V, E, Ve>),
    Ext(&'a Vertex<Ve>),
}

impl<'a, V, E, Ve> ExtVertex<'a, V, E, Ve> {
    pub fn len_out_edges(&self) -> usize {
        match self {
            Self::Core(core, v) => {
                let core_vertex = core.vertex(v.core_vidx);
                let core_out_edges = core_vertex.out_edges().len();
                let new_out_edges = v.ext_out_edges.len();
                core_out_edges + new_out_edges
            }
            Self::Ext(v) => v.out_edges().len(),
        }
    }

    // TODO: there is no reason why this doesn't return an ExactSizeIterator, but it doesn't compile, missing std implementation
    pub fn out_edges(&self) -> impl Iterator<Item = &OutEdge> {
        match self {
            Self::Core(core, v) => {
                let core_vertex = core.vertex(v.core_vidx);
                let core_out_edges = core_vertex.out_edges().into_iter();
                let new_out_edges = v.ext_out_edges.iter();
                core_out_edges.chain(new_out_edges)
            }
            Self::Ext(v) => v.out_edges().into_iter().chain((&[]).into_iter()),
        }
    }
}

pub struct CoreVertex<V, E, Ve> {
    core_vidx: VIdx,
    data: Ve,
    ext_out_edges: Vec<OutEdge>,
    ext_in_edges: Vec<InEdge>,
    p: PhantomData<(V, E)>,
}

impl<V, E, Ve> CoreVertex<V, E, Ve> {
    pub fn new(core_vidx: VIdx, data: Ve) -> Self {
        Self {
            core_vidx,
            data,
            ext_out_edges: Vec::new(),
            ext_in_edges: Vec::new(),
            p: PhantomData,
        }
    }
}
