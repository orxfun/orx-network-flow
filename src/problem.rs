use crate::commodities::{Commodity, CommodityData};
use crate::space_time::SpaceTime;
use crate::spaces::Spaces;
use crate::time::Time;
use crate::{commodities::Commodities, std_utils::MapKey};

pub struct Problem<S, K>
where
    S: MapKey,
    K: MapKey,
{
    spaces: Spaces<S>,
    commodities: Commodities<K>,
}

impl<S, K> Problem<S, K>
where
    S: MapKey,
    K: MapKey,
{
    pub fn commodity(&self, key: K) -> &CommodityData {
        todo!()
    }
}

// builder

pub struct ProblemBuilder<S, K>(Problem<S, K>)
where
    S: MapKey,
    K: MapKey;

impl<S, K> Default for ProblemBuilder<S, K>
where
    S: MapKey,
    K: MapKey,
{
    fn default() -> Self {
        Self(Problem {
            commodities: Default::default(),
            spaces: Default::default(),
        })
    }
}

impl<S, K> ProblemBuilder<S, K>
where
    S: MapKey,
    K: MapKey,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_commodity(
        &mut self,
        key: K,
        origin: S,
        ready_time: impl Into<Time>,
        destination: S,
        due_time: impl Into<Time>,
    ) {
        let ori_space = self.0.spaces.push(origin);
        let ori = SpaceTime::new(ori_space, ready_time.into());

        let des_space = self.0.spaces.push(destination);
        let des = SpaceTime::new(des_space, due_time.into());

        _ = self.0.commodities.push(key, ori, des);
    }

    pub fn finish(self) -> Problem<S, K> {
        self.0
    }
}
