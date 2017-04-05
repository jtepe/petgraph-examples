extern crate petgraph;

use petgraph::graph::DiGraph;
use petgraph::visit::Data;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::algo::FloatMeasure;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let mut g = DiGraph::<&str, u8>::new();
    let v0 = g.add_node("v0");
    let v1 = g.add_node("v1");
    g.extend_with_edges(&[(v0, v1, 10)]);
}

fn bellman_ford_algo<G>(g: G, s: G::NodeId) -> Result<HashMap<G::NodeId, bbG::EdgeWeight>, String>
    where G: IntoNodeIdentifiers + Data,
          G::NodeId: Eq + Hash,
          G::EdgeWeight: FloatMeasure
{

    Err("Rust".to_string())
}
