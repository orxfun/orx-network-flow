use crate::{space_time::SpaceTime, transports::Transport};

#[derive(derive_new::new)]
pub enum ComOdStDv {
    Transport(Transport),
    OriSt(SpaceTime),
    DesSt(SpaceTime),
}
