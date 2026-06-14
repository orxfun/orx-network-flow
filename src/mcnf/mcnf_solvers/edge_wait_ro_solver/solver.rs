use crate::Variant;
use crate::mcnf::McnfSol;
use crate::networks::ConnWaitNw;

#[derive(Default, Clone, Copy, Debug)]
pub struct EdgeWaitRoMcnfSolver;

impl EdgeWaitRoMcnfSolver {
    pub fn solve<V: Variant>(self, nw: &ConnWaitNw<'_, V>) -> McnfSol<V> {
        let named = cfg!(debug_assertions);
        todo!()
    }
}
