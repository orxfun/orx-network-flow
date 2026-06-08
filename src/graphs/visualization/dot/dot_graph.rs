use crate::graphs::{VIdx, visualization::dot::NodeSettings};
use alloc::format;
use alloc::string::String;

pub trait DotGraph {
    fn vertex_label(&self, v: VIdx) -> &str;

    fn vertex_tooltip(&self, v: VIdx) -> Option<&str>;

    fn vertex_settings(&self, v: VIdx) -> &NodeSettings;

    fn vertices(&self) -> impl Iterator<Item = VIdx>;

    fn edges(&self) -> impl Iterator<Item = (VIdx, VIdx)>;

    fn dot_string(&self) -> String {
        let mut dot = String::from("digraph G {\n");

        for v in self.vertices() {
            let label = self.vertex_label(v);
            let settings = self.vertex_settings(v);
            let tooltip = self.vertex_tooltip(v);

            let vertex = match tooltip {
                Some(tooltip) => {
                    format!("    {v} [label=\"{label}\"{settings} tooltip=\"{tooltip}\"];")
                }
                None => format!("    {v} [label=\"{label}\"{settings}];"),
            };

            dot.push_str(&vertex);
            dot.push('\n');
        }

        for (tail, head) in self.edges() {
            let edge = format!("    {} -> {};", tail, head);
            dot.push_str(&edge);
            dot.push('\n');
        }

        dot
    }
}
