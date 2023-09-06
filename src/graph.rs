use std::fs;
use std::collections::HashMap;
use crate::edge::Edge;
use crate::edge::build_edge;
use crate::edge::NodeId;
use crate::edge::EdgeId;
use crate::edge::Weight;


pub type Edgelist = Vec<HashMap<EdgeId, Edge>>;


// Reading the file
fn read_file(path: &str) -> String {
    // println!("Using file {}", path);
    fs::read_to_string(path)
        .expect("Should have been able to read the file")
}


// Creating data structure representing the graph and calculating indegree and outdegree of each node
fn create_graph(values: Vec<&str>, n_nodes : NodeId, k: usize) -> (Vec<HashMap<EdgeId, Edge>>, Vec<Weight>, Vec<Weight>, Vec<String>) {
    
    // Setup empty data structure
    let mut edgelist: Vec<HashMap<EdgeId, Edge>> = Vec::new();
    let empty : HashMap<EdgeId, Edge> = HashMap::new();
    let mut string_sequences: Vec<String> = Vec::new(); // Data structure to keep the string sequence related to an edge id
    for _ in 0..n_nodes {
        edgelist.push(empty.clone());
    }

    // Setup for checking flow condition
    let mut indeg = vec![0; n_nodes];
    let mut outdeg = vec![0; n_nodes];

    // Create the graph
    let rounds = (values).len() / 4;
    let mut id : EdgeId = 0;
    for i in 0..rounds {
        string_sequences.push(String::new());
        let node1: NodeId = values[i*4+1].parse().unwrap();
        let node2: NodeId = values[i*4+2].parse().unwrap();
        let nodeweight: Weight = values[i*4+3].parse().unwrap();
        // let ending = &values[i*4+4][(k-1)..].to_string();
        let edge = build_edge(id, node1, node2, nodeweight); //, (&values[i*4+4]).to_string());
        string_sequences[id] = (&values[i*4+4]).to_string();
        edgelist[node1].insert(edge.id, edge);
        id += 1;

        // Counting indegree and outdegree for checking flow condition
        indeg[node1] += nodeweight;
        outdeg[node2] += nodeweight;
    }
    (edgelist, indeg, outdeg, string_sequences)
}


// Check whether the flow condition holds. If not, produces an error 
fn flow_condition(indeg: Vec<Weight>, outdeg: Vec<Weight>) {
    for i in 0..indeg.len() {
        assert_eq!(indeg[i], outdeg[i], "Flow condition not satisfied");
    }
}








// Read the data and build the graph
pub fn build_graph(path: &str, k: usize) -> (Vec<HashMap<EdgeId, Edge>>, NodeId, Vec<String>) {

    // Reading the file
    let contents = read_file(path);
    
    // Setup
    let values: Vec<&str> = contents.split_whitespace().collect();
    let n_nodes = &values[0];
    let n_nodes : NodeId = n_nodes.parse().unwrap();    

    // Creating data structure representing the graph 
    let (edgelist, indeg, outdeg, string_sequences) = create_graph(values, n_nodes, k);

    // Check whether the flow condition holds. If not, produces an error 
    flow_condition(indeg, outdeg);
   
    (edgelist, n_nodes, string_sequences)
}


