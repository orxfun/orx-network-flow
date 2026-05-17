use num::Num;

pub trait Flow: Num {}

impl<N: Num> Flow for N {}
