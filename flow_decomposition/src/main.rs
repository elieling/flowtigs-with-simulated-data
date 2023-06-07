use std::fs;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
mod edge;
use crate::edge::Edge;
use crate::edge::build_edge;
use crate::edge::NodeId;
use crate::edge::EdgeId;
use crate::edge::Weight;




fn main() {
    // Choose the file you want to use
    // -------------------------------------------------------------- 
    // let path = "../data/short_k13.edgelist";
    // let path = "../data/test_k12.edgelist";
    let path = "../data/reference_k15.edgelist";
    // let path = "../data/long_k27.edgelist";
    // let path = "../data/fake.edgelist";
    // -------------------------------------------------------------- 

    // Reading the file
    println!("Using file {}", path);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
    println!("------------------------------------------");

    // Setup
    let values: Vec<&str> = contents.split_whitespace().collect();
    let n_nodes = &values[0];
    let n_nodes : NodeId = n_nodes.parse().unwrap();
    let mut edgelist: Vec<HashMap<EdgeId, Edge>> = Vec::new();
    let empty : HashMap<EdgeId, Edge> = HashMap::new();

    // Setup for checking flow condition
    let mut indeg = vec![0; n_nodes];
    let mut outdeg = vec![0; n_nodes];

    // Creating data structure representing the graph 
    for _ in 0..n_nodes {
        edgelist.push(empty.clone());
    }
    let rounds = (&values).len() / 4;
    let mut id : EdgeId = 0;
    for i in 0..rounds {
        let node1: NodeId = values[i*4+1].parse().unwrap();
        let node2: NodeId = values[i*4+2].parse().unwrap();
        let nodeweight: Weight = values[i*4+3].parse().unwrap();
        let e = build_edge(id, node1, node2, nodeweight, (&values[i*4+4]).to_string());
        edgelist[node1 as usize].insert(e.id, e);
        id += 1;

        // Counting for checking flow condition
        indeg[node1 as usize] += nodeweight;
        outdeg[node2 as usize] += nodeweight;
    }

    // Check flow condition
    let mut flow_condition = true;
    for i in 0..n_nodes {
        assert_eq!(indeg[i], outdeg[i], "Flow condition not satisfied");
        if indeg[i] != outdeg[i] {
            println!("PANIC WITH {}", i);
            flow_condition = false;
        }
    }
    if flow_condition {println!("Flow condition satisfied")}
    else {println!("ERROR: Flow condition noe satisfied")}

   
    // Printing all edges
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");
    for node in &edgelist {
        for (_, edge) in node {
            println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
        }
    }
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");


    //---------------------------------------------------------------------------
    // Edgelist is created from file and flow condition is checked.
    //---------------------------------------------------------------------------


    let mut queue : VecDeque<NodeId> = VecDeque::new(); // 
    let mut cycles : Vec<Vec<Edge>> = Vec::new();

    // Put all the nodes in the queue
    for i in 0..n_nodes {
        queue.push_back(i);
    }

    // Flow decomposition
    while !queue.is_empty() {
        // Going through all nodes as long as there are edges left on the graph
        let node : NodeId = queue.pop_front().unwrap();
        if edgelist[node].is_empty() {
            continue;
        }

        // Setting up for the loop
        let keys: Vec<_> = edgelist[node].keys().collect();
        let mut min_flow = edgelist[node][keys[0]].weight;
        let mut visited : HashSet<NodeId> = HashSet::new();
        let mut counter = 0;
        let mut new_node = node;
        let mut one_cycle : Vec<Edge> = Vec::new();

        // Find a cycle from the dbg and the flow of that cycle
        loop {
            // Collect all the edges of the chosen node
            let keys: Vec<_> = edgelist[new_node].keys().collect();

            // Find a valid edge
            while visited.contains(&edgelist[new_node][keys[counter]].end_node) && &edgelist[new_node][keys[counter]].end_node != &node {
                counter+=1;
            }
            let edge = &edgelist[new_node][keys[counter]];
            one_cycle.push(edge.clone());

            // Take next node
            new_node = edge.end_node;
            visited.insert(new_node);

            // Calculate the flow of the cycle
            min_flow = min(min_flow, edge.weight);

            // End the loop once a cycle is complete
            if new_node == node {
                break;
            } 
            counter = 0;
        }

        // Collect the newly found cycle
        cycles.push(one_cycle.clone());

        // Substract the cycle from the graph
        for edge in one_cycle {
            if let Some(edge) = edgelist[edge.start_node].get_mut(&edge.id) {
                edge.weight -= min_flow;
            }
            if edgelist[edge.start_node][&edge.id].weight == 0 {
                edgelist[edge.start_node].remove(&edge.id);
            }
        }

        // println!("*** Edgelist after removing ***");
        // for node in &edgelist {
        //     for (_, edge) in node {
        //         println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
        //     }
        // }
        // println!("*********");

        // Add node back to queue, so that it is handled as long as it has edges
        queue.push_back(node);
    }

    // Print the result
    println!("\n##### Next, the cycles: #####");
    let mut counter = 0;
    for cycle in cycles {
        println!("Cycle: {}", counter);
        counter += 1;
        for edge in cycle {
            println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
        }
    }


// Add doumentation and make cleaner.


}

