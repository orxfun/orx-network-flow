use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
fn extract_xlink_title(line: &str) -> Option<&str> {
    let start = line.find("xlink:title=\"")? + "xlink:title=\"".len();
    let end_rel = line[start..].find('"')?;
    Some(&line[start..start + end_rel])
}

#[cfg(feature = "std")]
fn overwrite_svg_titles_with_tooltips(svg: &str) -> String {
    let mut lines: Vec<String> = svg.lines().map(String::from).collect();

    for i in 1..lines.len() {
        let line = &lines[i];
        if !line.contains("<g id=\"a_") {
            continue;
        }

        let Some(tooltip) = extract_xlink_title(line) else {
            continue;
        };

        if !lines[i - 1].contains("<title>") {
            continue;
        }

        let indent = lines[i - 1].split("<title>").next().unwrap_or_default();
        lines[i - 1] = format!("{indent}<title>{tooltip}</title>");
    }

    lines.join("\n")
}

/// Graphviz emits tooltip text in xlink:title while keeping <title> as object id
/// (e.g. 0->1). Some viewers only show <title> on hover, so mirror tooltip text there.
#[cfg(feature = "std")]
pub fn fix_edge_labels_in_svg(svg_path: &str) -> Result<(), std::io::Error> {
    let svg = std::fs::read_to_string(svg_path)?;
    let svg = overwrite_svg_titles_with_tooltips(&svg);
    std::fs::write(svg_path, svg)
}
