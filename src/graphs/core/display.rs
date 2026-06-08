use crate::graphs::{Edge, core::GraphCore};
#[cfg(feature = "std")]
use alloc::{format, vec::Vec};
#[cfg(feature = "std")]
use core::fmt::Debug;
#[cfg(feature = "std")]
use std::println;

impl<Dv, De> GraphCore<Dv, De> {
    #[cfg(feature = "std")]
    pub fn print(&self)
    where
        De: Debug,
    {
        let headers = ["index", "tail", "head", "data"];
        let mut rows: Vec<[alloc::string::String; 4]> = Vec::with_capacity(self.edges.len());

        for (idx, edge) in self.edges.iter().enumerate() {
            rows.push([
                format!("{}", idx),
                format!("{}", edge.tail()),
                format!("{}", edge.head()),
                format!("{:?}", edge.data()),
            ]);
        }

        let mut widths = [0usize; 4];
        for i in 0..4 {
            let cell_width = rows.iter().map(|row| row[i].len()).max().unwrap_or(0);
            widths[i] = usize::max(headers[i].len(), cell_width);
        }

        println!(
            "{:>w0$} | {:>w1$} | {:>w2$} | {:<w3$}",
            headers[0],
            headers[1],
            headers[2],
            headers[3],
            w0 = widths[0],
            w1 = widths[1],
            w2 = widths[2],
            w3 = widths[3]
        );
        println!(
            "{}-+-{}-+-{}-+-{}",
            "-".repeat(widths[0]),
            "-".repeat(widths[1]),
            "-".repeat(widths[2]),
            "-".repeat(widths[3])
        );

        for row in &rows {
            println!(
                "{:>w0$} | {:>w1$} | {:>w2$} | {:<w3$}",
                row[0],
                row[1],
                row[2],
                row[3],
                w0 = widths[0],
                w1 = widths[1],
                w2 = widths[2],
                w3 = widths[3]
            );
        }
    }
}
