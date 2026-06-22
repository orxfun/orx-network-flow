use crate::graphs::visualization::dot::{EdgeSettings, VertexSettings};
use crate::graphs::{EIdx, Edge, Graph, VIdx};
use alloc::{format, string::String};
use core::fmt::Display;
#[cfg(feature = "std")]
use std::fs;
#[cfg(feature = "std")]
use std::process::Command;
#[cfg(feature = "std")]
use std::{io::Error, path::Path};

pub trait DotGraph {
    type G: Graph;

    fn vertex_label(&self, v: VIdx) -> impl Display;

    fn vertex_tooltip(&self, _: VIdx) -> Option<impl Display> {
        Option::<String>::None
    }

    fn vertex_settings(&self, v: VIdx) -> &VertexSettings;

    fn edge_label(&self, e: EIdx) -> impl Display;

    fn edge_tooltip(&self, _: EIdx) -> Option<impl Display> {
        Option::<String>::None
    }

    fn edge_settings(&self, e: EIdx) -> &EdgeSettings;

    fn graph_label(&self) -> Option<impl Display> {
        Option::<String>::None
    }

    fn graph(&self) -> &Self::G;

    fn vertices(&self) -> impl Iterator<Item = VIdx> {
        self.graph().vertex_indices()
    }

    fn edges(&self) -> impl Iterator<Item = (EIdx, VIdx, VIdx)> {
        self.graph().edge_indices().map(|e| {
            let edge = self.graph().edge(e);
            (e, edge.tail(), edge.head())
        })
    }

    fn dot_string(&self) -> String {
        let mut dot = String::from("digraph G {\n");

        if let Some(graph_label) = self.graph_label() {
            dot.push_str("    labelloc=\"b\";\n");
            dot.push_str("    labeljust=\"l\";\n");
            dot.push_str(&format!("    label=<{graph_label}>;\n"));
        }

        for v in self.vertices() {
            let label = self.vertex_label(v);
            let tooltip = self.vertex_tooltip(v);
            let settings = self.vertex_settings(v);

            let vertex = match tooltip {
                Some(tooltip) => {
                    format!("    {v} [label=\"{label}\"{settings} tooltip=\"{tooltip}\"];")
                }
                None => format!("    {v} [label=\"{label}\"{settings}];"),
            };

            dot.push_str(&vertex);
            dot.push('\n');
        }

        for (e, tail, head) in self.edges() {
            let label = self.edge_label(e);
            let tooltip = self.edge_tooltip(e);
            let settings = self.edge_settings(e);
            let edge = match tooltip {
                Some(tooltip) => {
                    format!(
                        "    {} -> {} [label=\"{label}\" {settings} tooltip=\"{tooltip}\"];",
                        tail, head
                    )
                }
                None => format!("    {} -> {} [label=\"{label}\" {settings}];", tail, head),
            };
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
        use crate::graphs::visualization::dot::edge_label_fix::fix_edge_labels_in_svg;

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

        fix_edge_labels_in_svg(svg_path)?;

        Ok(())
    }
}
