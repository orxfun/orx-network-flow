use crate::graph::GraphBuilder;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use crate::networks::aon::sources::Sources;
use crate::space_time::SpaceTime;
use crate::spaces::Space;
use crate::time::Time;
use crate::{Problem, Variant};

pub fn add_source_to_source_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let mut space = Space::from(usize::MAX);
    let mut tail = Time::from(i64::MAX);

    // TODO: avoid unsafe
    let graph = unsafe { &mut *(&mut builder.builder as *mut GraphBuilder<_, _>) };
    for st in builder.sources.iter_st_sorted() {
        match st.space() == space {
            false => {
                space = st.space();
                tail = st.time();
            }
            true => {
                let head = st.time();

                let i = builder.source_vidx(SpaceTime::new(space, tail));
                let j = builder.source_vidx(SpaceTime::new(space, head));
                let data = AonEdge::SourceSource(space, tail, head);
                graph.edge(data, i, j);

                tail = head;
            }
        }
    }
}
