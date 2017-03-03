extern crate petgraph;

use petgraph::graph::DiGraph;
use petgraph::visit::DfsPostOrder;

fn main() {
    let mut d = DiGraph::<&str, ()>::new();

    let v0 = d.add_node("v0");
    let v1 = d.add_node("v1");
    let v2 = d.add_node("v2");
    let v3 = d.add_node("v3");
    let v4 = d.add_node("v4");

    d.extend_with_edges(&[
        (v0, v2), (v0, v3),
        (v2, v1),
        (v1, v0),
        (v3, v4),
    ]);

    let mut r = DiGraph::<&str, ()>::new();

    let r0 = r.add_node("r0");
    let r1 = r.add_node("r1");
    let r2 = r.add_node("r2");
    let r3 = r.add_node("r3");
    let r4 = r.add_node("r4");

    r.extend_with_edges(&[
        (r2, r0), (r3, r0),
        (r1, r2),
        (r0, r1),
        (r4, r3),
    ]);

    let mut dfs = DfsPostOrder::new(&d, v0);
    let mut finish_order = Vec::new();

    while let Some(nx) = dfs.next(&d) {
        finish_order.push(nx);
    }

    println!("{:?}", finish_order);
}

