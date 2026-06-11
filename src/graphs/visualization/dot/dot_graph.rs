use crate::graphs::{Edge, Graph};
use crate::graphs::{VIdx, visualization::dot::NodeSettings};
use alloc::format;
use alloc::string::String;
use core::fmt::Display;
#[cfg(feature = "std")]
use std::fs;
#[cfg(feature = "std")]
use std::process::Command;
#[cfg(feature = "std")]
use std::{io::Error, path::Path};

pub trait DotGraph {
    fn vertex_label(&self, v: VIdx) -> impl Display;

    fn vertex_tooltip(&self, _: VIdx) -> Option<impl Display> {
        Option::<String>::None
    }

    fn vertex_settings(&self, v: VIdx) -> &NodeSettings;

    fn graph(&self) -> &impl Graph;

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.graph().vertex_indices()
    }

    fn edges(&self) -> impl Iterator<Item = (VIdx, VIdx)> {
        self.graph().edges().map(|e| (e.tail(), e.head()))
    }

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

        dot.push('}');

        dot
    }

    #[cfg(feature = "std")]
    fn create_dot_file(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        fs::write(path, self.dot_string())?;
        Ok(())
    }

    #[cfg(feature = "std")]
    fn create_svg_file(
        &self,
        dot_path: impl AsRef<Path> + Clone,
        svg_path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        self.create_dot_file(dot_path.clone())?;

        let dot_path = dot_path
            .as_ref()
            .as_os_str()
            .to_str()
            .expect("invalid dot file path");

        let svg_path = svg_path
            .as_ref()
            .as_os_str()
            .to_str()
            .expect("invalid svg file path");

        Command::new("dot")
            .args(["-Tsvg", dot_path, "-o", svg_path])
            .status()?;

        Ok(())
    }
}
