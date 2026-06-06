use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use orx_iterable::{IntoCloningIterable, Iterable};

pub fn add_source_to_transport_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();
    let p = &builder.p;

    for (ori, ori_sources) in builder.sources.sources_by_origins() {
        let sources_rev = ori_sources.iter().rev().into_iterable();

        let ori_transports = p.ori_des_sorted_transports.get(&ori);
        let od_transports = ori_transports.iter().flat_map(|x| x.values());
        let transports = od_transports.flat_map(|x| x.iter());
        for &t in transports {
            let dt = p.transport_by_idx(t).origin().time();
            if let Some(s) = sources_rev.iter().find(|s| dt >= s.0.time()) {
                let i = builder.source_vidx(s.0);
                let j = builder.transport_vidx(t);
                let data = AonEdge::SourceTransport;
                graph.edge(data, i, j);
            }
        }
    }
}
