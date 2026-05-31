use crate::graph::{Graph, visualization::dot::DotGraph};

#[test]
fn graph_validity() {
    let vertices = (0..4).map(|_| ());
    let mut builder = Graph::<(), ()>::builder(vertices);

    builder.edge((), 0, 1);
    builder.edge((), 0, 2);
    builder.edge((), 1, 2);
    builder.edge((), 1, 3);
    builder.edge((), 2, 3);

    builder.validate();
}

#[test]
fn graph_dot_export() {
    let vertices = (0..3).map(|_| ());
    let mut builder = Graph::<(), ()>::builder(vertices);
    builder.edge((), 0, 1);
    builder.edge((), 1, 2);

    let graph = builder.finish();
    let dot = graph.to_dot_string();

    assert!(dot.starts_with("digraph G {\n"));
    assert!(dot.ends_with('}'));
    assert!(dot.contains("    0;\n"));
    assert!(dot.contains("    1;\n"));
    assert!(dot.contains("    2;\n"));
    assert!(dot.contains("    0 -> 1;\n"));
    assert!(dot.contains("    1 -> 2;\n"));
}
