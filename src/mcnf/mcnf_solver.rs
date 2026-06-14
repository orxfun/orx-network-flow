use super::McnfSol;
use crate::{Problem, Variant, algorithm::Alg};

pub trait McnfSolver<V: Variant>
where
    Self: Alg<Output = McnfSol<V>>,
    for<'a> Self: Alg<Input<'a> = Problem<V>>,
{
}
