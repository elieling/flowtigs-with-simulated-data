use std::env::args;
// // use std::fs;
// use std::cmp::min;
// use std::collections::HashSet;
// // use std::collections::HashMap;
use std::collections::VecDeque;
mod edge;
use crate::edge::Edge;
// use crate::edge::build_edge;
// use crate::edge::NodeId;
// use crate::edge::EdgeId;
// use crate::edge::Weight;
mod graph;
use crate::graph::build_graph;
// use crate::graph::Edgelist;
mod flow;
use crate::flow::build_cycles;
use crate::flow::print_cycles;
use crate::flow::initialize_weight_of_neighbors_from;
mod cycle;
use crate::cycle::find_longest_subwalk;
mod uniqueness;
use crate::uniqueness::is_maximal;
use crate::uniqueness::unique_sequences;
use crate::uniqueness::create_parent_structure;
// use crate::cycle::longest_subwalk;
// use crate::cycle::get_former_index;
// use crate::cycle::ac_trie;
// use crate::cycle::try_removing;

// Logging
// use log::{info, warn};
// use simple_logger::SimpleLogger;
// SimpleLogger::new().init().unwrap();
// info!("Commencing yak shaving for {:?}", k);
// warn!("Unable to locate a razor, retrying");











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
    // let path = "../data/test_data/short.edgelist";
    // let path = "../data/test_data/sufpref.edgelist";
    // let path = "../data/test_data/outflow.edgelist";
    // let path = "../data/test_data/outflow_k2.edgelist";
    // let path = "../data/test_data/longer_k4.edgelist";

    // args
    let args: Vec<String> = args().collect();
    let path = &args[1];  // cargo run -- '../data/long_k27.edgelist'
    let k : usize = args[2].to_string().parse::<usize>().unwrap();
    // -------------------------------------------------------------- 

    // Read the data and build the graph
    let (edgelist, n_nodes) = build_graph(path, k);

    

    //---------------------------------------------------------------------------
    // Edgelist is created from file and flow condition is checked.
    // Next, flow decomposition algorithm.
    //---------------------------------------------------------------------------

    // BUild a data structure containing all the cycles in the dbg
    let cycles = build_cycles(edgelist.clone(), n_nodes, &edgelist);

    
    // Print the results
    print_cycles(&cycles);


    //---------------------------------------------------------------------------
    // Flow decomposition is done and the cycles are gathered.
    // Next, two-pointer algorithm.
    //---------------------------------------------------------------------------

    println!("************************************************************");

    // let mut safe_paths = Vec::new();
    // The paths as edges
    let mut safe_edge_paths = Vec::new();
    // The extra weight left corresponding to each path
    let mut extra_weight_of_paths = Vec::new();
    // The weight of neighbors of each node for edges leaving from that node
    let weight_of_neighbors_of_each_node = initialize_weight_of_neighbors_from(&edgelist);

    // Perform the algorithm on each cycle
    for cycle in cycles {

        // Initializing the vector for calculating paths in one cycle
        let mut one_cycle: VecDeque<Edge> = VecDeque::new();

        // If the cycle has only one edge, then the longest path in that cycle is that edge.
        if cycle.len() == 1 {
            // safe_paths.push(cycle[0].string.clone());
            one_cycle.push_back(cycle[0].clone());
            safe_edge_paths.push(one_cycle);
            extra_weight_of_paths.push(cycle[0].weight.clone());
        } else {
            // Setting up variables for a new cycle
            let mut i2 = 0; // Index of the second pointer
            // let mut sequence = String::from("");
            let mut weight_left = 0; // The amount of flow left for the path to be safe
            let mut former_weight = 0; // The weight of the first edge of the path is stored, to be able to move the first pointer
            // let mut safety = true; // The variable is true as long as the path is safe
            let mut neighbor_weights = Vec::new(); // Vector containing the flow leaving outside of the cycle for eachnode in the cycle
            
            // Initializing the neighbor_weights-vector 
            for i in 0..(cycle.len()) {
                let edge = &cycle[i];
                let weight_from_same_node = weight_of_neighbors_of_each_node[edge.start_node];
                neighbor_weights.push(weight_from_same_node - edge.weight);
            }

            // Calculating the safe paths for this cycle
            for i in 0..(cycle.len()) {
                (i2, weight_left, former_weight) = find_longest_subwalk(&mut one_cycle, weight_left, 
                    former_weight, &mut neighbor_weights, &mut safe_edge_paths, 
                    i, i2, &cycle, &mut extra_weight_of_paths);
            
            } 
        }

        // safe_edge_paths.push(one_cycle);

    }

    let parents = create_parent_structure(&edgelist);

    // println!("\n+++++ Then, the safe paths as sequences: +++++");
    // let mut counter = 0;
    // for sequence in &safe_paths {
    //     println!("Path {}:", counter);
    //     counter += 1;
    //     println!("{}", sequence);
    // }

   
    println!("\n++++++++ Then, the safe paths as edges: ++++++++");
    let mut counter = 0;
    for sequence in &safe_edge_paths {
        println!("Path {}:", counter);
        for edge in sequence {
            print!("{} ", edge.string);
        }
        println!("");
        println!("Maximal? {}\n", is_maximal(&sequence, &edgelist, extra_weight_of_paths[counter], &parents, &weight_of_neighbors_of_each_node));
        counter += 1;
    }

    
    // println!("\nwwwww Then, the weights: wwwww");
    // let mut counter = 0;
    // for extra in &extra_weight_of_paths {
    //     println!("Weight: {}:", counter);
    //     counter += 1;
    //     println!("{}", extra);
    // }

    // println!("\nwwwww Then, the weights of neighbors: wwwww");
    // let mut counter = 0;
    // for node in &weight_of_neighbors_of_each_node {
    //     println!("Weight: {}:", counter);
    //     counter += 1;
    //     println!("{}", node);
    // }

    let safe_paths = unique_sequences(safe_edge_paths, k, &extra_weight_of_paths, &edgelist, weight_of_neighbors_of_each_node);

    println!("\n++++++++ Then, the safe paths as final unique strings: ++++++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        println!("Path {}:", counter);
        println!("{} ", sequence);
        counter += 1;
    }

   
}

// Check maximality also in other direction. Test with more complicated graphs
// Function for Parent structure exist, then use it for maximality in other direction

// cargo run -- '../data/test_data/outflow.edgelist'
