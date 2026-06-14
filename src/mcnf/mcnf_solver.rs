use super::McnfSol;
use crate::{Problem, Variant, algorithm::Alg};

pub trait McnfSolver<'a, V: Variant>
where
    Self: Alg<Output = McnfSol<V>>,
    Self: Alg<Input = &'a Problem<V>>,
{
}
