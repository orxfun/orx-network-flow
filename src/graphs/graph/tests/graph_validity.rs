use crate::graphs::{Graph, VIdx, graph::visualization::dot::DotGraph};

#[test]
fn graph_validity() {
    let vertices = (0..4).map(|_| ());
    let mut builder = Graph::<(), ()>::builder(vertices);

    builder.edge((), VIdx::from(0), VIdx::from(1));
    builder.edge((), VIdx::from(0), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(3));
    builder.edge((), VIdx::from(2), VIdx::from(3));

    builder.validate();
}

#[test]
fn graph_dot_export() {
    let vertices = (0..3).map(|_| ());
    let mut builder = Graph::<(), ()>::builder(vertices);
    builder.edge((), VIdx::from(0), VIdx::from(1));
    builder.edge((), VIdx::from(1), VIdx::from(2));

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
