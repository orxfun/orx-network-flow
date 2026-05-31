use orx_network_flow::{DotGraph, Graph};

fn main() {
    let vertices = (0..4).map(|_| ());
    let mut builder = Graph::<(), ()>::builder(vertices);

    builder.edge((), 0, 1);
    builder.edge((), 0, 2);
    builder.edge((), 1, 2);
    builder.edge((), 1, 3);
    builder.edge((), 2, 3);

    builder.validate();
    let graph = builder.finish();

    graph.print();

    println!("\n\nDOT\n{}", graph.to_dot_string());
}
