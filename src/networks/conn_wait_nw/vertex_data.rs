use crate::transports::Transport;

pub enum ConnWaitVertex {
    Transport(Transport),
}
