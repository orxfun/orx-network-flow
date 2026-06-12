use crate::{space_time::SpaceTime, transports::Transport};

pub enum ConnWaitVertex {
    Transport(Transport),
    ReadyOri(SpaceTime),
    DueDes(SpaceTime),
}
