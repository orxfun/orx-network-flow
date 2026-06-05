use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use crate::space_time::SpaceTime;
use crate::{spaces::Space, time::Time};

pub fn add_sink_to_sink_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let mut des = Space::from(usize::MAX);
    let mut due1 = Time::from(i64::MAX);

    let (builder, graph) = builder.split_graph();
    for st in builder.sinks.iter_st_sorted() {
        match st.space() == des {
            false => {
                des = st.space();
                due1 = st.time();
            }
            true => {
                let due2 = st.time();

                let i = builder.sink_vidx(SpaceTime::new(des, due1));
                let j = builder.sink_vidx(SpaceTime::new(des, due2));
                let data = AonEdge::SinkSink;
                graph.edge(data, i, j);

                due1 = due2;
            }
        }
    }
}
