use crate::flow_units::FlowUnit;
use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, VecVertex, Vertex};
use crate::networks::ConnWaitNw;
use crate::networks::conn_wait_nw::ConnWaitEdge;
use crate::{IdxCore, Solution, Time, Transport, Variant, VecTransport};
use alloc::vec::Vec;
use orx_priority_queue::{
    BinaryHeapOfIndices, PriorityQueue, PriorityQueueDecKey, ResTryDecreaseKeyOrPush,
};

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
    let mut visited = VecVertex::new_filled(g.v(), false);
    let mut pred = VecVertex::new_filled(g.v(), VIdx::from(0));
    let mut path = Vec::new();

    for c in p.commodities.values() {
        let (ro, dd) = (c.origin(), c.destination());
        match (nw.ro_to_v.get(&ro), nw.dd_to_v.get(&dd)) {
            (Some(&s), Some(&t)) => {
                let found = shortest_path::<V>(&g, &mut heap, &mut visited, &mut pred, s, t);
                assert!(found);
                build_transport_path(p.len_transports(), &mut pred, &mut path, s, t);
            }
            _ => unreachable!(),
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

fn shortest_path<V: Variant>(
    g: &GrSp<V>,
    heap: &mut Heap,
    visited: &mut VecVertex<bool>,
    pred: &mut VecVertex<VIdx>,
    s: VIdx,
    t: VIdx,
) -> bool {
    heap.clear();
    visited.iter_mut().for_each(|x| *x = false);

    heap.push(s, Time::zero());

    while let Some((v, cost)) = heap.pop() {
        match v == t {
            true => return true,
            false => {
                let vertex = g.vertex(v);
                let out_indices = vertex.out_edges();
                let with_cap = out_indices.map(|e| g.edge(e)).filter(is_edge_open::<V>);
                let not_visited = with_cap.filter(|e| !visited[e.head()]);

                let myo_edges: Vec<_> = vertex
                    .out_edges()
                    .map(|e| g.edge(e))
                    .filter(is_edge_open::<V>)
                    .filter(|e| !visited[e.head()])
                    .collect();

                for edge in not_visited {
                    match heap.try_decrease_key_or_push(&edge.head(), cost + edge.data().time) {
                        ResTryDecreaseKeyOrPush::Decreased | ResTryDecreaseKeyOrPush::Pushed => {
                            pred[edge.head()] = v;
                        }
                        ResTryDecreaseKeyOrPush::Unchanged => {}
                    }
                }

                visited[v] = true;
            }
        }
    }

    false
}

fn build_transport_path(
    len_transports: usize,
    pred: &mut VecVertex<VIdx>,
    path: &mut Vec<Transport>,
    s: VIdx,
    t: VIdx,
) {
    let v_to_t = |v: VIdx| match v.into_inner() < len_transports {
        true => Some(Transport::from(v.into_inner())),
        false => None,
    };

    path.clear();

    let mut curr = t;
    while curr != s {
        if let Some(t) = v_to_t(curr) {
            path.push(t);
        }
        curr = pred[curr];
    }

    if let Some(t) = v_to_t(curr) {
        path.push(t);
    }
}
