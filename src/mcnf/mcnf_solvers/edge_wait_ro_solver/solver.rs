use crate::mcnf::McnfSol;
use crate::networks::ConnWaitNw;
use crate::{Problem, Variant};

#[derive(Default, Clone, Copy, Debug)]
pub struct EdgeWaitRoMcnfSolver;

impl EdgeWaitRoMcnfSolver {
    pub fn solve<V: Variant>(self, nw: &ConnWaitNw<'_, V>) -> McnfSol<V> {
        let named = true;
        todo!()
    }
}
