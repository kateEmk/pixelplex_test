mod tests;

use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::fs::*;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;


#[derive(Debug)]
pub struct Edge<T> {
    to_node: u32,
    edge: T
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
        else { println!("This node exists") };
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
            edge: edge_value
        };
        if let Some(vector_edges) = self.nodes.get_mut(&begin_node){
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
                let line: String = i.to_string() + " " + &*edge.to_node.to_string() + " " +
                    &*edge.edge.to_string() + "\n";
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
                    edge: parts[2].parse::<T>().unwrap(),
                };
                self.add_relation_to_node(node_id, edge.to_node, edge.edge);
            }
        }
    }

    pub fn print_graph(&self) {
        for relation in self.nodes.keys() {
            println!("Vertex id: {}", relation);
            println!("\tRelations: ");
            for edges in &self.nodes[relation] {
                println!("\t\tTo node: {}\tEdge value: {}", edges.to_node, edges.edge);
            }
        }
    }

}