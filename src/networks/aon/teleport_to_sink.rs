use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;

pub fn add_teleport_sink_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();

    for (tidx, commodities) in builder.sinks.iter_tidx_and_commodities() {
        let j = builder.tidx_to_vidx(tidx);
        for &c in commodities {
            let i = builder.teleport_vidx(c);
            let data = AonEdge::TeleportSink;
            graph.edge(data, i, j);
        }
    }
}
