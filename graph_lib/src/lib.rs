use std::borrow::BorrowMut;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::fs::*;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;


#[derive(Debug)]
pub struct Edge<T> {
    to_node: u32,
    value: T
}

#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: HashMap<u32, Vec<Edge<T>>>,
}

impl<T> Graph<T>
where T: Display + FromStr
{
    pub fn new_graph() -> Graph<T> {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_id: u32) {
        if !(self.nodes.contains_key(&node_id)) {
            let relation = Vec::new();
            self.nodes.insert(node_id, relation);
        }
        else { panic!("This node exists") };
    }

    pub fn delete_node(&mut self, node_id: u32) {
        &self.nodes.remove(&node_id);
        for v in self.nodes.values_mut() {
            v.retain(|edge| edge.to_node != node_id);
        }
    }

    pub fn add_relation_to_node(&mut self, begin_node: u32, end_node: u32,
                                 edge_value: T) {
        if !(
            self.nodes.contains_key(&begin_node) && self.nodes.contains_key(&end_node)
        ) {
            panic!("This node doesn't exist");
        }

        let relation = Edge {
            to_node: end_node,
            value: edge_value
        };

        if let Some(vector_edges) = self.nodes.get_mut(&begin_node){
            for v in vector_edges.iter() {
                if v.to_node == end_node {
                    panic!("This relation exists");
                }
            }
            vector_edges.push(relation);
        }
    }

    pub fn delete_relation(&mut self, from_node: u32, to_node: u32) {
        if let Some(v) = self.nodes.get_mut(&from_node){
            v.retain(|edge| edge.to_node != to_node);
        }

        if let Some(v) = self.nodes.get_mut(&to_node){
            v.retain(|edge| edge.to_node != from_node);
        }
    }

    pub fn bfs(&self, start: u32) -> Vec<u32> {
        let mut queue = VecDeque::new();
        let mut visited = Vec::new();

        queue.push_back(start);
        visited.push(start);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            println!("Visiting node {}", node);

            for neighbor in &self.nodes[&node] {
                if !visited.contains(&neighbor.to_node) {
                    visited.push(neighbor.to_node);
                    queue.push_back(neighbor.to_node);
                }
            }
        }
        return visited;
    }

    pub fn serialize(&self, filename: String) {
        let mut file = File::create(filename).expect("create failed");

        let mut hashmap_keys = self.nodes.keys().collect::<Vec<_>>();
        hashmap_keys.sort();

        for i in hashmap_keys.iter(){
            file.write(i.to_string().as_ref()).expect("Unable to write data");
            file.write(b"\n").expect("Unable to write data");
        }

        file.write(b"#\n").expect("Unable to write data");

        for i in hashmap_keys.iter(){
            for edge in &self.nodes[i] {
                let line: String = i.to_string() + "\t" + &*edge.to_node.to_string() + "\t" +
                    &*edge.value.to_string() + "\n";
                file.write(line.as_bytes()).expect("Unable to write data");
            }
        }
    }

    pub fn deserialize(&mut self, filename: String)
    where <T as FromStr>::Err: Debug {
        let file = File::open(filename).expect("Unable to load file");
        let mut read_nodes: bool = true;

        let buf = BufReader::new(file).lines();
        for line in buf{
            if line.as_ref().unwrap() == "#" {
                read_nodes = false;
                continue;
            }
            if read_nodes {
                let int_value = line.unwrap().parse::<u32>().unwrap();
                self.add_node(int_value);
            } else {
                let parts = Vec::from_iter(line.as_ref().unwrap().split_whitespace());
                let node_id = parts[0].parse::<u32>().unwrap();
                let edge = Edge {
                    to_node: parts[1].parse::<u32>().unwrap(),
                    value: parts[2].parse::<T>().unwrap(),
                };
                self.add_relation_to_node(node_id, edge.to_node, edge.value);
            }
        }
    }

    pub fn print_graph(&self) {
        for relation in self.nodes.keys() {
            println!("Vertex id: {}", relation);
            println!("\tRelations: ");
            for edges in &self.nodes[relation] {
                println!("\t\tTo node: {}\tEdge value: {}", edges.to_node, edges.value);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{Edge, Graph};

    #[test]
    #[should_panic(expected = "This node exists")]
    fn add_node() {
        let mut graph = Graph::<u32>::new_graph();
        graph.add_node(1);
        graph.add_node(1);
    }

    #[test]
    fn delete_node() {
        let mut graph = Graph::<u32>::new_graph();
        graph.add_node(1);
        graph.add_node(2);
        graph.delete_node(2);
        let result: bool = false;
        assert_eq!(graph.nodes.contains_key(&2), result);
    }

    #[test]
    #[should_panic(expected = "This node doesn't exist")]
    fn add_relation_to_node_1test() {
        let mut graph = Graph::<u32>::new_graph();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_relation_to_node(3, 2, 5);
    }

    #[test]
    #[should_panic(expected = "This relation exists")]
    fn add_relation_to_node_2test() {
        let mut graph = Graph::<u32>::new_graph();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_relation_to_node(1, 2, 5);
        graph.add_relation_to_node(1, 2, 5);
    }

    #[test]
    fn delete_relation() {
        let mut graph = Graph::<u32>::new_graph();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_relation_to_node(1, 2, 5);
        graph.delete_relation(1, 2);
        assert_eq!(graph.nodes.get_mut(&1).unwrap().len(), 0);
    }

    #[test]
    fn serialize_deserialize_graph() {
        let mut graph: Graph<u32> = Graph::new_graph();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_relation_to_node(1, 2, 5);
        graph.add_relation_to_node(1, 3, 10);
        graph.serialize("graph-test.rs".to_string());

        let mut new_graph: Graph<u32> = Graph::new_graph();
        new_graph.deserialize("graph-test.rs".to_string());
        assert_eq!(new_graph.print_graph(), graph.print_graph());
    }

}