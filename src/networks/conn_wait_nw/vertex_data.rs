use crate::{SpaceTime, Transport};

pub enum ConnWaitVertex {
    Transport(Transport),
    ReadyOri(SpaceTime),
    DueDes(SpaceTime),
}
