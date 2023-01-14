use std::str::FromStr;
pub use graph_lib;
use crate::graph_lib::Graph;

fn main() {
    let mut graph: Graph<u32> = Graph::new_graph();
    let filename: String = "graph.txt".to_string();
    graph.deserialize(filename);

    graph.print_graph();
}