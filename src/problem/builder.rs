use crate::problem::Problem;
use crate::problem::variant::Variant;
use crate::space_time::SpaceTime;
use crate::time::Time;

pub struct ProblemBuilder<V: Variant>(Problem<V>);

impl<V: Variant> Default for ProblemBuilder<V> {
    fn default() -> Self {
        Self(Problem {
            commodities: Default::default(),
            spaces: Default::default(),
        })
    }
}

impl<V: Variant> ProblemBuilder<V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_commodity(
        &mut self,
        key: V::K,
        origin: V::S,
        ready_time: impl Into<Time>,
        destination: V::S,
        due_time: impl Into<Time>,
    ) {
        let ori_space = self.0.spaces.push(origin);
        let ori = SpaceTime::new(ori_space, ready_time.into());

        let des_space = self.0.spaces.push(destination);
        let des = SpaceTime::new(des_space, due_time.into());

        _ = self.0.commodities.push(key, ori, des);
    }

    pub fn finish(self) -> Problem<V> {
        self.0
    }
}
