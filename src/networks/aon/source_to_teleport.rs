use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;

pub fn add_source_to_teleport_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let (builder, graph) = builder.split_graph();

    for (sidx, commodities) in builder.sources.iter_sidx_and_commodities() {
        let i = builder.sidx_to_vidx(sidx);
        for &c in commodities {
            let j = builder.teleport_vidx(c);
            let data = AonEdge::SourceTeleport;
            graph.edge(data, i, j);
        }
    }
}
