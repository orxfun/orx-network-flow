use orx_network_flow::Graph;

fn main() {
    let mut builder = Graph::<(), ()>::builder(4, |_| ());

    builder.edge((), 0, 1);
    builder.edge((), 0, 2);
    builder.edge((), 1, 2);
    builder.edge((), 1, 3);
    builder.edge((), 2, 3);

    builder.validate();
    let graph = builder.finish();

    graph.print();
}
