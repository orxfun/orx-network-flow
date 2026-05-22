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

impl<S> Default for Problem<S>
where
    S: MapKey,
{
    fn default() -> Self {
        Self {
            spaces: Default::default(),
            commodities: Default::default(),
        }
    }
}

impl<S> Problem<S>
where
    S: MapKey,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_commodity(
        &mut self,
        origin: S,
        ready_time: impl Into<Time>,
        destination: S,
        due_time: impl Into<Time>,
    ) -> Commodity {
        let ori_space = self.spaces.push(origin);
        let ori = SpaceTime::new(ori_space, ready_time.into());

        let des_space = self.spaces.push(destination);
        let des = SpaceTime::new(des_space, due_time.into());

        self.commodities.push(ori, des)
    }
}
