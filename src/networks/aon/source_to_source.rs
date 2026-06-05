use crate::Variant;
use crate::networks::aon::AonEdge;
use crate::networks::aon::network_builder::AonNetworkBuilder;
use crate::space_time::SpaceTime;
use crate::{spaces::Space, time::Time};

pub fn add_source_to_source_edges<V: Variant>(builder: &mut AonNetworkBuilder<'_, V>) {
    let mut ori = Space::from(usize::MAX);
    let mut ready1 = Time::from(i64::MAX);

    let (builder, graph) = builder.split_graph();
    // for st in builder.sources.iter_st_sorted() {
    //     match st.space() == ori {
    //         false => {
    //             ori = st.space();
    //             ready1 = st.time();
    //         }
    //         true => {
    //             let ready2 = st.time();

    //             let i = builder.source_vidx(SpaceTime::new(ori, ready1));
    //             let j = builder.source_vidx(SpaceTime::new(ori, ready2));
    //             let data = AonEdge::SourceSource;
    //             graph.edge(data, i, j);

    //             ready1 = ready2;
    //         }
    //     }
    // }
}
