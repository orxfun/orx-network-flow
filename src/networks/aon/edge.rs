use crate::{spaces::Space, time::Time};

pub enum AonEdge {
    SourceSource(Space, Time, Time),
}
