use super::Solution;
use crate::{Problem, Variant};

pub struct SolutionBuilder<'a, V: Variant> {
    p: &'a Problem<V>,
    solution: Solution<V>,
}

impl<'a, V: Variant> SolutionBuilder<'a, V> {
    pub fn new(p: &'a Problem<V>) -> Self {
        let commodity_paths = (0..p.len_commodities())
            .map(|_| Default::default())
            .collect();
        let transport_loads = (0..p.len_transports())
            .map(|_| Default::default())
            .collect();
        let solution = Solution::new(commodity_paths, transport_loads);
        Self { p, solution }
    }

    pub fn finish(self) -> Solution<V> {
        self.solution
    }

    // mut
}
