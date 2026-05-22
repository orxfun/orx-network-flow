use crate::spaces::{SpaceData, Spaces};
use crate::{commodities::Commodities, std_utils::MapKey};

pub struct Problem<S, Sd>
where
    S: MapKey,
    Sd: SpaceData,
{
    spaces: Spaces<S, Sd>,
    commodities: Commodities,
}

impl<S, Sd> Default for Problem<S, Sd>
where
    S: MapKey,
    Sd: SpaceData,
{
    fn default() -> Self {
        Self {
            spaces: Default::default(),
            commodities: Default::default(),
        }
    }
}
