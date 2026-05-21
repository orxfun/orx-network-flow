use crate::{commodities::Commodities, spaces::Spaces, std_utils::MapKey};

pub struct Problem<S, Sd>
where
    S: MapKey,
{
    spaces: Spaces<S, Sd>,
    commodities: Commodities,
}
