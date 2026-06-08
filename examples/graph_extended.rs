use orx_network_flow::IdxCore;
use orx_network_flow::graphs::core::GraphCore;
use orx_network_flow::graphs::extended::GraphExtended;
use orx_network_flow::graphs::visualization::dot::{AsDotGraph, DotGraph};
use orx_network_flow::graphs::{Edge, Graph, VIdx};

fn main() {
    let vertices = (0..4).map(|_| ());
    let mut builder = GraphCore::<(), ()>::builder(vertices);

    builder.edge((), VIdx::from(0), VIdx::from(1));
    builder.edge((), VIdx::from(0), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(3));
    builder.edge((), VIdx::from(2), VIdx::from(3));

    let core = builder.finish();

    let core_vertices = core.vertex_indices().map(|v: VIdx| v.to_string());
    let core_edges = core
        .edges()
        .map(|e| e.head().into_inner() + e.tail().into_inner());
    let mut builder = GraphExtended::<_, String, usize>::builder(&core, core_vertices, core_edges);

    let v4 = builder.node("x".to_string());
    let v5 = builder.node("y".to_string());
    builder.edge(45, v4, v5);
    builder.edge(14, VIdx::from(1), v4);
    builder.edge(52, v5, VIdx::from(2));
    builder.edge(52, VIdx::from(3), VIdx::from(2));

    let extended = builder.finish();

    let dot = extended.as_dot_graph();
    dot.create_svg_file("target/graph_extended.dot", "target/graph_extended.svg")
        .unwrap();

    println!("\n\nDOT\n{}", dot.dot_string());
}
