use crate::{problem::Problem, std_utils::MapKey};
use alloc::{format, vec, vec::Vec};
use core::fmt::Debug;

impl<S, K> Debug for Problem<S, K>
where
    S: MapKey,
    K: MapKey,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let space_entries = self.spaces.entries();
        let mut space_keys_by_index = vec![None; space_entries.len()];

        for (space, key) in &space_entries {
            space_keys_by_index[usize::from(*space)] = Some(*key);
        }

        writeln!(f, "Problem")?;
        writeln!(f, "  spaces: {}", self.len_spaces())?;
        writeln!(f, "  commodities: {}", self.len_commodities())?;

        writeln!(f)?;
        writeln!(f, "Spaces")?;
        let spaces_header_index = "index";
        let spaces_header_key = "key";
        let spaces_index_width = usize::max(
            spaces_header_index.len(),
            space_entries
                .iter()
                .map(|(space, _)| format!("{}", usize::from(*space)).len())
                .max()
                .unwrap_or(0),
        );
        let spaces_key_width = usize::max(
            spaces_header_key.len(),
            space_entries
                .iter()
                .map(|(_, key)| format!("{:?}", key).len())
                .max()
                .unwrap_or(0),
        );

        writeln!(
            f,
            "{:>index_w$} | {:<key_w$}",
            spaces_header_index,
            spaces_header_key,
            index_w = spaces_index_width,
            key_w = spaces_key_width
        )?;
        writeln!(
            f,
            "{}-+-{}",
            "-".repeat(spaces_index_width),
            "-".repeat(spaces_key_width)
        )?;
        for (space, key) in space_entries {
            writeln!(
                f,
                "{:>index_w$} | {:<key_w$}",
                usize::from(space),
                format!("{:?}", key),
                index_w = spaces_index_width,
                key_w = spaces_key_width
            )?;
        }

        writeln!(f)?;
        writeln!(f, "Commodities")?;
        let commodity_entries: Vec<_> = self.commodities.entries().collect();

        let commodity_header = [
            "index",
            "key",
            "origin",
            "destination",
            "ready_time",
            "due_time",
        ];

        let mut commodity_rows: Vec<[alloc::string::String; 6]> =
            Vec::with_capacity(commodity_entries.len());
        for (commodity, key, data) in commodity_entries {
            let origin = data.origin();
            let destination = data.destination();

            let ori_idx = usize::from(origin.space());
            let des_idx = usize::from(destination.space());

            let ori_key = space_keys_by_index.get(ori_idx).and_then(|x| *x);
            let des_key = space_keys_by_index.get(des_idx).and_then(|x| *x);

            commodity_rows.push([
                format!("{}", usize::from(commodity)),
                format!("{:?}", key),
                format!("{:?}", ori_key),
                format!("{:?}", des_key),
                format!("{:?}", origin.time()),
                format!("{:?}", destination.time()),
            ]);
        }

        let mut widths = [0usize; 6];
        for i in 0..6 {
            let cell_width = commodity_rows
                .iter()
                .map(|row| row[i].len())
                .max()
                .unwrap_or(0);
            widths[i] = usize::max(commodity_header[i].len(), cell_width);
        }

        writeln!(
            f,
            "{:>w0$} | {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$}",
            commodity_header[0],
            commodity_header[1],
            commodity_header[2],
            commodity_header[3],
            commodity_header[4],
            commodity_header[5],
            w0 = widths[0],
            w1 = widths[1],
            w2 = widths[2],
            w3 = widths[3],
            w4 = widths[4],
            w5 = widths[5]
        )?;
        writeln!(
            f,
            "{}-+-{}-+-{}-+-{}-+-{}-+-{}",
            "-".repeat(widths[0]),
            "-".repeat(widths[1]),
            "-".repeat(widths[2]),
            "-".repeat(widths[3]),
            "-".repeat(widths[4]),
            "-".repeat(widths[5])
        )?;

        for row in &commodity_rows {
            writeln!(
                f,
                "{:>w0$} | {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$}",
                row[0],
                row[1],
                row[2],
                row[3],
                row[4],
                row[5],
                w0 = widths[0],
                w1 = widths[1],
                w2 = widths[2],
                w3 = widths[3],
                w4 = widths[4],
                w5 = widths[5]
            )?;
        }

        Ok(())
    }
}
