use crate::graph::VIdx;
use crate::networks::aon::sources::Sources;
use crate::networks::aon::{AonEdge, AonVertex};
use crate::spaces::Space;
use crate::time::Time;
use crate::{Problem, Variant, graph::GraphBuilder};

pub fn add_source_to_source_edges<V: Variant>(
    builder: &mut GraphBuilder<AonVertex, AonEdge>,
    p: &Problem<V>,
    sources: &Sources,
) {
    let mut space = Space::from(usize::MAX);
    let mut tail = Time::from(i64::MAX);

    for st in sources.iter_st_sorted() {
        match st.space() == space {
            false => {
                space = st.space();
                tail = st.time();
            }
            true => {
                let head = st.time();

                tail = head;
            }
        }
    }
}
