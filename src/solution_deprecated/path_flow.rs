use crate::{Variant, solution_deprecated::Path};

pub struct PathFlow<V: Variant> {
    path: Path,
    flow: V::F,
}
