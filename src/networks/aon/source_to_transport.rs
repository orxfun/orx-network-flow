use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use core::cmp::Ordering;

pub fn add_source_to_transport_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();
    let p = &builder.p;

    for (ori, sources) in builder.sources.chunks_by_origins() {
        if let Some(ori_transports) = p.ori_des_sorted_transports.get(&ori) {
            for (_des, transports) in ori_transports {
                let mut sources = sources.iter();
                let mut source = sources.next();

                for &t in transports {
                    let dt = p.transport_by_idx(t).origin().time();
                    loop {
                        match source {
                            Some(s) => match s.1.cmp(&dt) {
                                Ordering::Equal => {
                                    let i = builder.source_vidx(s.0);
                                    let j = builder.transport_vidx(t);
                                    let data = AonEdge::SourceTransport;
                                    graph.edge(data, i, j);
                                    break;
                                }
                                Ordering::Less => source = sources.next(),
                                Ordering::Greater => break,
                            },
                            None => break,
                        }
                    }
                }
            }
        }
    }
}
