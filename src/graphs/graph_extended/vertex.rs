use crate::graphs::{EIdx, Graph, InEdge, OutEdge, VIdx, Vertex};
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

    pub fn len_in_edges(&self) -> usize {
        match self {
            Self::Core(core, v) => {
                let core_vertex = core.vertex(v.core_vidx);
                let core_in_edges = core_vertex.in_edges().len();
                let new_in_edges = v.ext_in_edges.len();
                core_in_edges + new_in_edges
            }
            Self::Ext(v) => v.in_edges().len(),
        }
    }

    pub fn in_edges(&self) -> impl Iterator<Item = &InEdge> {
        match self {
            Self::Core(core, v) => {
                let core_vertex = core.vertex(v.core_vidx);
                let core_in_edges = core_vertex.in_edges().into_iter();
                let new_in_edges = v.ext_in_edges.iter();
                core_in_edges.chain(new_in_edges)
            }
            Self::Ext(v) => v.in_edges().into_iter().chain((&[]).into_iter()),
        }
    }
}

pub enum ExtVertexMut<'a, V, E, Ve> {
    Core(&'a Graph<V, E>, &'a mut CoreVertex<V, E, Ve>),
    Ext(&'a mut Vertex<Ve>),
}

impl<'a, V, E, Ve> ExtVertexMut<'a, V, E, Ve> {
    pub fn add_out_edge(&mut self, edges_idx: EIdx, head: VIdx, head_in_edge_idx: usize) {
        match self {
            Self::Core(_, v) => {
                let out_edge = OutEdge::new(edges_idx, head, head_in_edge_idx);
                v.ext_out_edges.push(out_edge);
            }
            Self::Ext(v) => v.add_out_edge(edges_idx, head, head_in_edge_idx),
        }
    }

    pub fn add_in_edge(&mut self, edges_idx: EIdx, tail: VIdx, tail_out_edge_idx: usize) {
        match self {
            Self::Core(_, v) => {
                let in_edge = InEdge::new(edges_idx, tail, tail_out_edge_idx);
                v.ext_in_edges.push(in_edge);
            }
            Self::Ext(v) => v.add_in_edge(edges_idx, tail, tail_out_edge_idx),
        }
    }
}

pub struct CoreVertex<V, E, Ve> {
    pub(super) core_vidx: VIdx,
    pub(super) data: Ve,
    pub(super) ext_out_edges: Vec<OutEdge>,
    pub(super) ext_in_edges: Vec<InEdge>,
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
