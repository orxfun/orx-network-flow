use crate::Variant;
use crate::networks::core::AonEdge;
use crate::networks::core::network_builder::AonNetworkBuilder;
use core::cmp::Ordering;

pub fn add_transport_to_sink_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();
    let p = &builder.p;

    for (des, sinks) in builder.sinks.chunks_by_destinations() {
        if let Some(des_transports) = p.des_ori_sorted_transports.get(&des) {
            for (_ori, transports) in des_transports {
                let mut sinks = sinks.iter();
                let mut sink = sinks.next();

                for &t in transports {
                    let due = p.transport_by_idx(t).destination().time();
                    loop {
                        match sink {
                            Some(s) => match s.1.cmp(&due) {
                                Ordering::Equal => {
                                    let i = builder.transport_vidx(t);
                                    let j = builder.sink_vidx(s.0);
                                    let data = AonEdge::TransportSink;
                                    graph.edge(data, i, j);
                                    break;
                                }
                                Ordering::Less => sink = sinks.next(),
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
