use crate::flow_units::FlowUnit;
use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, Vertex};
use crate::networks::ConnWaitNw;
use crate::networks::conn_wait_nw::ConnWaitEdge;
use crate::{Solution, Time, Variant};
use orx_priority_queue::{BinaryHeapOfIndices, PriorityQueue, PriorityQueueDecKey};

pub struct Output<V: Variant> {
    pub edge_flows: VecEdge<V::F>,
    pub solution: Solution<V>,
}

impl<V: Variant> Output<V> {
    pub fn create(nw: &ConnWaitNw<'_, V>, edge_flows: VecEdge<V::F>) -> Self {
        let solution = create_solution(nw, &edge_flows);
        Self {
            edge_flows,
            solution,
        }
    }
}

fn create_solution<V: Variant>(nw: &ConnWaitNw<'_, V>, edge_flows: &VecEdge<V::F>) -> Solution<V> {
    let (p, g) = (nw.p, &nw.g);
    let mut builder = Solution::builder(p.len_commodities());
    let b = &mut builder;

    let tail = |e: EIdx| g.vertex(g.edge(e).tail());
    let tail_t = |e: EIdx| p.transport_by_idx(tail(e).data().get_t().expect("t"));
    let edge_cost = |e: EIdx, x: &ConnWaitEdge| match x {
        ConnWaitEdge::Connect | ConnWaitEdge::Exit => tail_t(e).duration(),
        _ => Time::zero(),
    };
    let edge_flow = |e: EIdx, x: &ConnWaitEdge| match x {
        ConnWaitEdge::Bypass(_) => Default::default(),
        _ => edge_flows[e],
    };
    let map_edge = |e: EIdx, x: &ConnWaitEdge| EdgeData::new(edge_cost(e, x), edge_flow(e, x));

    let g = g.map(|_, _| (), map_edge);
    let mut heap = BinaryHeapOfIndices::with_index_bound(g.v());

    for c in p.commodities.values() {
        let (ro, dd) = (c.origin(), c.destination());
        match (nw.ro_to_v.get(&ro), nw.dd_to_v.get(&dd)) {
            (Some(&s), Some(&t)) => {
                //
                shortest_path::<V>(&g, &mut heap, s, t)
            }
            _ => todo!(),
        }
    }

    builder.finish()
}

#[derive(derive_new::new)]
struct EdgeData<V: Variant> {
    time: Time,
    flow: V::F,
}
type GrSp<V> = GraphCore<(), EdgeData<V>>;
type Heap = BinaryHeapOfIndices<VIdx, Time>;

#[inline]
fn is_edge_open<V: Variant>(edge: &&EdgeCore<EdgeData<V>>) -> bool {
    edge.data().flow.is_pos()
}

fn shortest_path<V: Variant>(g: &GrSp<V>, heap: &mut Heap, s: VIdx, t: VIdx) {
    heap.clear();
    heap.push(s, Time::zero());

    while let Some((v, cost)) = heap.pop() {
        match v == t {
            true => {
                //
                todo!()
            }
            false => {
                let vertex = g.vertex(v);
                let out_indices = vertex.out_edges();
                let out_edges = out_indices.map(|e| g.edge(e)).filter(is_edge_open::<V>);
                for edge in out_edges {
                    heap.try_decrease_key_or_push(&edge.head(), cost + edge.data().time);
                }
            }
        }
    }
}
