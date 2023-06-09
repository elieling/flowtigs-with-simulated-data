// // use std::fs;
// use std::cmp::min;
// use std::collections::HashSet;
// // use std::collections::HashMap;
// use std::collections::VecDeque;
mod edge;
use crate::edge::Edge;
// use crate::edge::build_edge;
// use crate::edge::NodeId;
// use crate::edge::EdgeId;
use crate::edge::Weight;
mod graph;
use crate::graph::build_graph;
use crate::graph::Edgelist;
mod flow;
use crate::flow::build_cycles;
use crate::flow::print_cycles;


// FUNCTION FOR SINGLE SAFE WALK
// 

// Function to get the next edge in the cycle. 
// Returns the edge and its index in the cycle.
fn next_edge(cycle: &Vec<Edge>, index: usize) -> (Edge, usize) {
    let limit = cycle.len() - 1;
    if index == limit {
        return (cycle[0].clone(), 0);
    }
    return (cycle[index+1].clone(), index+1);
}

// Function to test the safety when adding an edge to the path. 
// Returns whether it is safe and the remaining weight.  
fn step (cycle: &Vec<Edge>, index: usize, weight: Weight, edgelist: &Edgelist) -> (bool, Weight) {
    // println!("STEP");
    let edge = cycle[index].clone();
    // let start_node = edge.start_node;
    let id = edge.id;
    // let (_, next_id) = next_edge(cycle, index);
    let neighbors = &edgelist[edge.start_node];
    let mut weight = weight;
    // println!("Edge: {}", edge.string);

    let mut neighbor_weight = 0;
    for (_, neigh) in neighbors {
        // println!("neighbor {}, neigh_id {}, id {}", neigh.string, neigh.id, id);
        if neigh.id != id {
            neighbor_weight += neigh.weight;
            // println!("neigh_weight: {}", neighbor_weight);
        }
    }

    weight -= neighbor_weight;
    let mut safety = false;
    if weight > 0 {safety = true};
    (safety, weight)
}

// Function that calculates the longest subwalk starting from a particular edge.
// Returns a String of the longest path starting from the node.
fn longest_subwalk(cycle: &Vec<Edge>, index: usize, edgelist: &Edgelist) -> String {
    let mut longest_path = String::from("");
    let mut index = index;
    let original_edge = &cycle[index];
    longest_path += &original_edge.string;
    let mut weight_left = original_edge.weight;
    loop {
        let (edge, next_index) = next_edge(&cycle, index);
        index = next_index;
        let (safety, weight) = step(&cycle, index, weight_left, &edgelist); 
        // println!("weight: {}", weight);
        if safety {
            longest_path.push(edge.last_char());
            weight_left = weight;
            if edge.id == original_edge.id {break;}
        } else {break;}
    }
    longest_path
}



fn main() {
    // Choose the file you want to use
    // -------------------------------------------------------------- 
    // let path = "../data/short_k13.edgelist";
    // let path = "../data/test_k12.edgelist";
    // let path = "../data/reference_k15.edgelist";
    // let path = "../data/long_k27.edgelist";
    // let path = "../data/ecoli_k12.edgelist";
    // let path = "../data/fake.edgelist";

    // Test files
    let path = "../data/test_data/short.edgelist";
    // -------------------------------------------------------------- 

    // Read the data and build the graph
    let (edgelist, n_nodes) = build_graph(path);


    //---------------------------------------------------------------------------
    // Edgelist is created from file and flow condition is checked.
    // Next, flow decomposition algorithm.
    //---------------------------------------------------------------------------

    // BUild a data structure containing all the cycles in the dbg
    let cycles = build_cycles(edgelist.clone(), n_nodes);

    
    // Print the results
    print_cycles(&cycles);


    //---------------------------------------------------------------------------
    // Flow decomposition is done and the cycles are gathered.
    // Next, two-pointer algorithm.
    //---------------------------------------------------------------------------

    println!("************************************************************");

    let mut safe_paths = Vec::new();

    // Perform the algorithm on each cycle
    for cycle in cycles {
        if cycle.len() == 1 {
            // safe_paths.push(vec![cycle[0].clone()]);
            safe_paths.push(cycle[0].string.clone());
        } else {
            for i in 0..cycle.len() {
                safe_paths.push(longest_subwalk(&cycle, i, &edgelist));
                // break;
            }
        }
        // break;
    }

    println!("\n+++++ Then, the safe paths: +++++");
    let mut counter = 0;
    for sequence in safe_paths {
        println!("Path {}:", counter);
        counter += 1;
        println!("{}", sequence);
        // for sequence in path {
            // println!("{}", edge.last_char());
            // println!("{}", sequence);
        // }
    }
}


