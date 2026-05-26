use crate::problem::{Problem, variant::Variant};
use crate::spaces::VecSpace;
use alloc::{format, vec::Vec};
use core::fmt::Debug;

impl<V: Variant> Debug for Problem<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let space_entries: Vec<_> = self.spaces.entries().collect();
        let mut space_keys_by_index: VecSpace<_> = (0..space_entries.len()).map(|_| None).collect();

        for (space, key) in &space_entries {
            space_keys_by_index[*space] = Some(*key);
        }

        writeln!(f, "Problem")?;
        writeln!(f, "  spaces: {}", self.len_spaces())?;
        writeln!(f, "  commodities: {}", self.len_commodities())?;
        writeln!(f, "  transports: {}", self.len_transports())?;

        writeln!(f)?;
        writeln!(f, "Spaces")?;
        let spaces_header_index = "index";
        let spaces_header_key = "key";
        let spaces_index_width = usize::max(
            spaces_header_index.len(),
            space_entries
                .iter()
                .map(|(space, _)| format!("{}", *space).len())
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
            let space_cell = format!("{}", space);
            let key_cell = format!("{:?}", key);
            writeln!(
                f,
                "{:>index_w$} | {:<key_w$}",
                space_cell,
                key_cell,
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
            "amount",
        ];

        let mut commodity_rows: Vec<[alloc::string::String; 7]> =
            Vec::with_capacity(commodity_entries.len());
        for (commodity, key, data) in commodity_entries {
            let origin = data.origin();
            let destination = data.destination();

            let ori_idx = origin.space();
            let des_idx = destination.space();

            let ori_key = space_keys_by_index.get(ori_idx).and_then(|x| *x);
            let des_key = space_keys_by_index.get(des_idx).and_then(|x| *x);

            commodity_rows.push([
                format!("{}", commodity),
                format!("{:?}", key),
                format!("{:?}", ori_key),
                format!("{:?}", des_key),
                format!("{:?}", origin.time()),
                format!("{:?}", destination.time()),
                format!("{:?}", data.amount()),
            ]);
        }

        let mut widths = [0usize; 7];
        for i in 0..7 {
            let cell_width = commodity_rows
                .iter()
                .map(|row| row[i].len())
                .max()
                .unwrap_or(0);
            widths[i] = usize::max(commodity_header[i].len(), cell_width);
        }

        writeln!(
            f,
            "{:>w0$} | {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$} | {:<w6$}",
            commodity_header[0],
            commodity_header[1],
            commodity_header[2],
            commodity_header[3],
            commodity_header[4],
            commodity_header[5],
            commodity_header[6],
            w0 = widths[0],
            w1 = widths[1],
            w2 = widths[2],
            w3 = widths[3],
            w4 = widths[4],
            w5 = widths[5],
            w6 = widths[6]
        )?;
        writeln!(
            f,
            "{}-+-{}-+-{}-+-{}-+-{}-+-{}-+-{}",
            "-".repeat(widths[0]),
            "-".repeat(widths[1]),
            "-".repeat(widths[2]),
            "-".repeat(widths[3]),
            "-".repeat(widths[4]),
            "-".repeat(widths[5]),
            "-".repeat(widths[6])
        )?;

        for row in &commodity_rows {
            writeln!(
                f,
                "{:>w0$} | {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$} | {:<w6$}",
                row[0],
                row[1],
                row[2],
                row[3],
                row[4],
                row[5],
                row[6],
                w0 = widths[0],
                w1 = widths[1],
                w2 = widths[2],
                w3 = widths[3],
                w4 = widths[4],
                w5 = widths[5],
                w6 = widths[6]
            )?;
        }

        writeln!(f)?;
        writeln!(f, "Transports")?;
        let transport_entries: Vec<_> = self.transports.entries().collect();

        let transport_header = [
            "index",
            "key",
            "origin",
            "destination",
            "dep_time",
            "arr_time",
            "capacity",
        ];

        let mut transport_rows: Vec<[alloc::string::String; 7]> =
            Vec::with_capacity(transport_entries.len());
        for (transport, key, data) in transport_entries {
            let origin = data.origin();
            let destination = data.destination();

            let ori_idx = origin.space();
            let des_idx = destination.space();

            let ori_key = space_keys_by_index.get(ori_idx).and_then(|x| *x);
            let des_key = space_keys_by_index.get(des_idx).and_then(|x| *x);

            let ori_cell = ori_key
                .map(|x| format!("{:?}", x))
                .unwrap_or_else(|| "-".into());
            let des_cell = des_key
                .map(|x| format!("{:?}", x))
                .unwrap_or_else(|| "-".into());

            transport_rows.push([
                format!("{}", transport),
                format!("{:?}", key),
                ori_cell,
                des_cell,
                format!("{:?}", origin.time()),
                format!("{:?}", destination.time()),
                format!("{:?}", data.capacity()),
            ]);
        }

        let mut widths = [0usize; 7];
        for i in 0..7 {
            let cell_width = transport_rows
                .iter()
                .map(|row| row[i].len())
                .max()
                .unwrap_or(0);
            widths[i] = usize::max(transport_header[i].len(), cell_width);
        }

        writeln!(
            f,
            "{:>w0$} | {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$} | {:<w6$}",
            transport_header[0],
            transport_header[1],
            transport_header[2],
            transport_header[3],
            transport_header[4],
            transport_header[5],
            transport_header[6],
            w0 = widths[0],
            w1 = widths[1],
            w2 = widths[2],
            w3 = widths[3],
            w4 = widths[4],
            w5 = widths[5],
            w6 = widths[6]
        )?;
        writeln!(
            f,
            "{}-+-{}-+-{}-+-{}-+-{}-+-{}-+-{}",
            "-".repeat(widths[0]),
            "-".repeat(widths[1]),
            "-".repeat(widths[2]),
            "-".repeat(widths[3]),
            "-".repeat(widths[4]),
            "-".repeat(widths[5]),
            "-".repeat(widths[6])
        )?;

        for row in &transport_rows {
            writeln!(
                f,
                "{:>w0$} | {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$} | {:<w6$}",
                row[0],
                row[1],
                row[2],
                row[3],
                row[4],
                row[5],
                row[6],
                w0 = widths[0],
                w1 = widths[1],
                w2 = widths[2],
                w3 = widths[3],
                w4 = widths[4],
                w5 = widths[5],
                w6 = widths[6]
            )?;
        }

        Ok(())
    }
}
