use std::fs;
use std::collections::HashMap;
// use super::edge;
use crate::edge::Edge;
use crate::edge::build_edge;
use crate::edge::NodeId;
use crate::edge::EdgeId;
use crate::edge::Weight;


pub type Edgelist = Vec<HashMap<EdgeId, Edge>>;


// Reading the file
fn read_file(path: &str) -> String {
    // println!("Using file {}", path);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    // println!("With text:\n{contents}");
    // println!("------------------------------------------");
    contents
}


// Creating data structure representing the graph and calculating indegree and outdegree of each node
fn create_graph(values: Vec<&str>, n_nodes : NodeId) -> (Vec<HashMap<EdgeId, Edge>>, Vec<Weight>, Vec<Weight>) {
    
    // Setup empty data structure
    let mut edgelist: Vec<HashMap<EdgeId, Edge>> = Vec::new();
    let empty : HashMap<EdgeId, Edge> = HashMap::new();
    for _ in 0..n_nodes {
        edgelist.push(empty.clone());
    }

    // Setup for checking flow condition
    let mut indeg = vec![0; n_nodes];
    let mut outdeg = vec![0; n_nodes];

    // Create the graph
    let rounds = (&values).len() / 4;
    let mut id : EdgeId = 0;
    for i in 0..rounds {
        let node1: NodeId = values[i*4+1].parse().unwrap();
        let node2: NodeId = values[i*4+2].parse().unwrap();
        let nodeweight: Weight = values[i*4+3].parse().unwrap();
        let e = build_edge(id, node1, node2, nodeweight, (&values[i*4+4]).to_string());
        edgelist[node1 as usize].insert(e.id, e);
        id += 1;

        // Counting indegree and outdegree for checking flow condition
        indeg[node1 as usize] += nodeweight;
        outdeg[node2 as usize] += nodeweight;
    }
    (edgelist, indeg, outdeg)
}


// Check whether the flow condition holds. If not, produces an error 
fn flow_condition(indeg: Vec<Weight>, outdeg: Vec<Weight>) {
    // let mut flow_condition = true;
    for i in 0..indeg.len() {
        assert_eq!(indeg[i], outdeg[i], "Flow condition not satisfied");
        // if indeg[i] != outdeg[i] {
        //     println!("PANIC WITH {}", i);
        //     // flow_condition = false;
        // }
    }
    println!("Flow condition satisfied")
    // if flow_condition {println!("Flow condition satisfied")}
    // else {println!("ERROR: Flow condition noe satisfied")}
}








// Read the data and build the graph
pub fn build_graph(path: &str) -> (Vec<HashMap<EdgeId, Edge>>, NodeId) {

    // Reading the file
    let contents = read_file(path);
    
    // Setup
    let values: Vec<&str> = contents.split_whitespace().collect();
    let n_nodes = &values[0];
    let n_nodes : NodeId = n_nodes.parse().unwrap();    

    // Creating data structure representing the graph 
    let (edgelist, indeg, outdeg) = create_graph(values, n_nodes);

    // Check whether the flow condition holds. If not, produces an error 
    flow_condition(indeg, outdeg);
   
    // Printing all edges
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");
    for node in &edgelist {
        for (_, edge) in node {
            println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
        }
    }
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");
    (edgelist, n_nodes)
}


