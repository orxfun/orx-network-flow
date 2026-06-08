use orx_network_flow::graphs::VIdx;
use orx_network_flow::graphs::core::GraphCore;
use orx_network_flow::graphs::visualization::dot::{AsDotGraph, DotGraph};

fn main() {
    let vertices = (0..4).map(|_| ());
    let mut builder = GraphCore::<(), ()>::builder(vertices);

    builder.edge((), VIdx::from(0), VIdx::from(1));
    builder.edge((), VIdx::from(0), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(3));
    builder.edge((), VIdx::from(2), VIdx::from(3));

    let graph = builder.finish();

    let dot = graph.as_dot_graph();
    dot.create_svg_file("target/graph_core.dot", "target/graph_core.svg")
        .unwrap();

    graph.print();

    println!("\n\nDOT\n{}", dot.dot_string());
}
