use crate::Variant;
use crate::networks::core::CoreNwEdge;
use crate::networks::core::network_builder::CoreNwBuilder;

pub fn add_waiting_edges<V: Variant>(builder: &mut CoreNwBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();
    let p = &builder.p;

    for (_ori, des_sorted_transports) in &p.ori_des_sorted_transports {
        for (_des, sorted_transports) in des_sorted_transports {
            let tails = sorted_transports.iter().copied();
            let heads = sorted_transports.iter().copied().skip(1);
            for (tail, head) in tails.zip(heads) {
                let data = CoreNwEdge::Waiting;
                let i = builder.transport_vidx(tail);
                let j = builder.transport_vidx(head);
                graph.edge(data, i, j);
            }
        }
    }
}
