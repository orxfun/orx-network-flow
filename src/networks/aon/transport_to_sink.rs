use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use orx_iterable::{IntoCloningIterable, Iterable};

pub fn add_transport_to_sink_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();
    let p = &builder.p;

    for (des, des_sinks) in builder.sinks.chunks_by_destinations() {
        let sources_rev = des_sinks.iter().rev().into_iterable();

        let ori_transports = p.ori_des_sorted_transports.get(&des);
        let od_transports = ori_transports.iter().flat_map(|x| x.values());
        let transports = od_transports.flat_map(|x| x.iter());
        for &t in transports {
            let dt = p.transport_by_idx(t).origin().time();
            if let Some(s) = sources_rev.iter().find(|s| dt >= s.0.time()) {
                let i = builder.source_vidx(s.0);
                let j = builder.transport_vidx(t);
                let data = AonEdge::SourceTransport;
                // graph.edge(data, i, j);
            }
        }
    }
}
