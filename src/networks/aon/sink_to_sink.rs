use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use crate::space_time::SpaceTime;
use crate::{spaces::Space, time::Time};

pub fn add_sink_to_sink_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let mut space = Space::from(usize::MAX);
    let mut tail = Time::from(i64::MAX);

    let (builder, graph) = builder.split_ref();
    for st in builder.sinks.iter_st_sorted() {
        match st.space() == space {
            false => {
                space = st.space();
                tail = st.time();
            }
            true => {
                let head = st.time();

                let i = builder.sink_vidx(SpaceTime::new(space, tail));
                let j = builder.sink_vidx(SpaceTime::new(space, head));
                let data = AonEdge::SinkSink(space, tail, head);
                graph.edge(data, i, j);

                tail = head;
            }
        }
    }
}
