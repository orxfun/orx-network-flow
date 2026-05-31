use crate::commodities::Commodity;
use crate::graph::GraphBuilder;
use crate::graph_builders::activity_on_node::indexer::Indexer;
use crate::graph_builders::activity_on_node::{edge::EdgeData, vertex::VertexData};
use crate::indices::IdxCore;
use crate::transports::Transport;
use crate::{Problem, Variant};

pub fn edges_source_to_transport<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
) {
    for (ori, sorted_commodities) in &prob.ori_sorted_commodities {
        if let Some(des_sorted_transports) = prob.ori_des_sorted_transports.get(ori) {
            for (_des, sorted_transports) in des_sorted_transports {
                let commodities_rev = sorted_commodities.iter().rev().copied();
                let transports_rev = sorted_transports.iter().rev().copied();

                connect_transports_of_od(prob, builder, indexer, commodities_rev, transports_rev);
            }
        }
    }
}

fn connect_transports_of_od<V: Variant>(
    prob: &Problem<V>,
    builder: &mut GraphBuilder<VertexData, EdgeData>,
    indexer: &Indexer,
    mut commodities_rev: impl Iterator<Item = Commodity>,
    mut transports_rev: impl Iterator<Item = Transport>,
) -> Option<()> {
    // no edges once we complete traversing transports
    let mut curr_t = transports_rev.next()?;

    // connect one commodity per iteration
    loop {
        // no edges once we complete traversing commodities
        let c = commodities_rev.next()?;

        let (t, next_t) = find_transport_for_commodity(prob, &mut transports_rev, curr_t, c);

        if let Some(t) = t {
            let data = EdgeData::SourceToTransport(c, t);
            let tail = indexer.source_idx(c).into_inner();
            let head = indexer.transport_idx(t).into_inner();
            builder.edge(data, tail, head);
        }

        // no edges once we complete traversing transports
        curr_t = next_t?;
    }
}

fn find_transport_for_commodity<V: Variant>(
    prob: &Problem<V>,
    transports_rev: &mut impl Iterator<Item = Transport>,
    curr_t: Transport,
    c: Commodity,
) -> (Option<Transport>, Option<Transport>) {
    let ready = prob.commodity_by_idx(c).origin().time();
    let departure = prob.transport_by_idx(curr_t).origin().time();

    if ready > departure {
        // no transport can be connected to c
        // but curr_t can still be connected to earlier commodities
        return (None, Some(curr_t));
    }

    let mut curr_t = curr_t;
    loop {
        match transports_rev.next() {
            Some(next_t) => {
                let departure = prob.transport_by_idx(next_t).origin().time();
                match ready <= departure {
                    // next_t can also connect to c, so it must be preferred
                    true => curr_t = next_t,
                    // curr_t can connect to c; next_t can be connected to earlier commodities
                    false => return (Some(curr_t), Some(next_t)),
                }
            }
            // curr_t is the earliest transport and can connect to c
            None => return (Some(curr_t), None),
        }
    }
}
