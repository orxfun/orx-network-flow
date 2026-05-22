use crate::commodities::Commodity;
use crate::space_time::SpaceTime;
use crate::spaces::Spaces;
use crate::time::Time;
use crate::{commodities::Commodities, std_utils::MapKey};

pub struct Problem<S>
where
    S: MapKey,
{
    spaces: Spaces<S>,
    commodities: Commodities,
}

impl<S> Problem<S> where S: MapKey {}

// builder

pub struct ProblemBuilder<S>(Problem<S>)
where
    S: MapKey;

impl<S> Default for ProblemBuilder<S>
where
    S: MapKey,
{
    fn default() -> Self {
        Self(Problem {
            commodities: Default::default(),
            spaces: Default::default(),
        })
    }
}

impl<S> ProblemBuilder<S>
where
    S: MapKey,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_commodity(
        &mut self,
        origin: S,
        ready_time: impl Into<Time>,
        destination: S,
        due_time: impl Into<Time>,
    ) -> Commodity {
        let ori_space = self.0.spaces.push(origin);
        let ori = SpaceTime::new(ori_space, ready_time.into());

        let des_space = self.0.spaces.push(destination);
        let des = SpaceTime::new(des_space, due_time.into());

        self.0.commodities.push(ori, des)
    }

    pub fn finish(self) -> Problem<S> {
        self.0
    }
}
