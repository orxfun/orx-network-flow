use core::ops::Neg;
use num::{Num, Zero};

pub trait Cost: Num + Zero + Neg<Output = Self> {}

impl<C: Num + Zero + Neg<Output = Self>> Cost for C {}
