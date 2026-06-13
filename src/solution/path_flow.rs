use crate::{Variant, solution::Path};

pub struct PathFlow<V: Variant> {
    path: Path,
    flow: V::F,
}
