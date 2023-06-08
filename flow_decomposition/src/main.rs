// use std::fs;
use std::cmp::min;
use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::VecDeque;
mod edge;
use crate::edge::Edge;
// use crate::edge::build_edge;
use crate::edge::NodeId;
// use crate::edge::EdgeId;
// use crate::edge::Weight;
mod graph;
use crate::graph::build_graph;





fn main() {
    // Choose the file you want to use
    // -------------------------------------------------------------- 
    // let path = "../data/short_k13.edgelist";
    // let path = "../data/test_k12.edgelist";
    let path = "../data/reference_k15.edgelist";
    // let path = "../data/long_k27.edgelist";
    // let path = "../data/fake.edgelist";
    // -------------------------------------------------------------- 

    // Read the data and build the graph
    let (mut edgelist, n_nodes) = build_graph(path);


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
        let mut new_node : NodeId = node;
        let mut flow = Vec::new();
        flow.push(min_flow);
        let mut one_cycle : Vec<Edge> = Vec::new();

        // Find a cycle from the dbg and the flow of that cycle
        'single_flow: loop {
            // Collect all the edges of the chosen node
            let keys: Vec<_> = edgelist[new_node].keys().collect();

            // Find a valid edge
            while visited.contains(&edgelist[new_node][keys[counter]].end_node) && &edgelist[new_node][keys[counter]].end_node != &node {
                counter+=1;

                // Backtrack if there are no valid edge left
                if counter == keys.len() {
                    counter = 0;
                    let edge = one_cycle.pop().unwrap();
                    new_node = edge.start_node;
                    flow.pop();
                    continue 'single_flow;
                }
            }
            let edge = &edgelist[new_node][keys[counter]];
            one_cycle.push(edge.clone());

            // Take next node
            new_node = edge.end_node;
            visited.insert(new_node);

            // Calculate the flow of the cycle
            min_flow = min(flow[flow.len()-1], edge.weight);
            flow.push(min_flow);

            // End the loop once a cycle is complete
            if new_node == node {
                break;
            } 
            counter = 0;
        }

        // Collect the newly found cycle
        cycles.push(one_cycle.clone());

        // Substract the cycle from the graph
        min_flow = flow[flow.len()-1];
        for edge in one_cycle {
            if let Some(edge) = edgelist[edge.start_node].get_mut(&edge.id) {
                edge.weight -= min_flow;
            }
            if edgelist[edge.start_node][&edge.id].weight == 0 {
                edgelist[edge.start_node].remove(&edge.id);
            }
        }

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


}

