#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpaceKind {
    #[default]
    Basic,
    Euclidean,
    Geographic,
}
