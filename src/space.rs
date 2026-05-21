#[derive(Clone, Copy, Debug)]
pub struct Space(usize);

impl Space {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }
}
