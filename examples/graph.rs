use orx_network_flow::{DotGraph, Graph, VIdx};

fn main() {
    let vertices = (0..4).map(|_| ());
    let mut builder = Graph::<(), ()>::builder(vertices);

    builder.edge((), VIdx::from(0), VIdx::from(1));
    builder.edge((), VIdx::from(0), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(2));
    builder.edge((), VIdx::from(1), VIdx::from(3));
    builder.edge((), VIdx::from(2), VIdx::from(3));

    builder.validate();
    let graph = builder.finish();

    graph.print();

    println!("\n\nDOT\n{}", graph.to_dot_string());
}
