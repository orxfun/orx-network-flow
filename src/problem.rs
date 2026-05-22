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
