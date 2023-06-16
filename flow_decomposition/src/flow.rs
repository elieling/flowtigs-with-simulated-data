use std::collections::HashSet;
use std::collections::HashMap;
use crate::edge::Edge;
use crate::edge::NodeId;
use crate::edge::EdgeId;
use crate::edge::Weight;
use crate::graph::Edgelist;
use std::cmp::min;
use std::collections::VecDeque;


pub fn initialize_weight_of_neighbors_from(edgelist: &Edgelist) -> Vec<Weight> {
    let mut weights_of_neighbors = Vec::new();
    for i in 0..edgelist.len() {
        weights_of_neighbors.push(0);
        for (_, edge) in &edgelist[i] {
            weights_of_neighbors[i] += edge.weight;
        }
    }


    weights_of_neighbors
}


// Put all the nodes in the queue
fn build_queue(n_nodes: NodeId) -> VecDeque<NodeId> {
    let mut queue = VecDeque::new(); 
    for i in 0..n_nodes {
        queue.push_back(i);
    }
    queue
}




// Print the results
pub fn print_cycles(cycles: &Vec<Vec<Edge>>) {
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


// BUild a data structure containing all the cycles in the dbg
pub fn build_cycles(mut edgelist: Vec<HashMap<EdgeId, Edge>>, n_nodes: NodeId) -> Vec<Vec<Edge>> {
    
    let mut cycles : Vec<Vec<Edge>> = Vec::new();

    // Put all the nodes in the queue
    let mut queue = build_queue(n_nodes);

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
    cycles
}