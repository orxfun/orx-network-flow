use crate::graphs::visualization::dot::node_settings::NodeSettings;

pub trait DotVertex {
    fn label(&self) -> &str;

    fn settings(&self) -> &NodeSettings;

    fn tooltip(&self) -> Option<&str>;
}
