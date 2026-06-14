use crate::{Variant, algorithm::Alg, mcnf::McnfSol};

pub trait McnfSolver<V: Variant>: Alg<Output = McnfSol<V>> {}
