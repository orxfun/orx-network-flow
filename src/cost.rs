use core::ops::Neg;
use num::{Num, Zero};

pub trait Cost: Clone + Copy + Num + Zero + Neg<Output = Self> {}

impl<C: Clone + Copy + Num + Zero + Neg<Output = Self>> Cost for C {}
