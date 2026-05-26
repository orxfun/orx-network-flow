use num::{Num, Zero};

pub trait Cost: Num + Zero {}

impl<C: Num + Zero> Cost for C {}
