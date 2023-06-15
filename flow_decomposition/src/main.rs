use std::env::args;
// // use std::fs;
// use std::cmp::min;
use std::collections::HashSet;
// // use std::collections::HashMap;
use std::collections::VecDeque;
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
mod cycle;
use crate::cycle::longest_subwalk;
use crate::cycle::get_former_index;
// use crate::cycle::ac_trie;
// use crate::cycle::try_removing;



fn unique_sequences(safe_edge_paths: Vec<VecDeque<Edge>>, k: usize) -> HashSet<String> {
    let mut safe_paths = HashSet::new();
    for mut sequence in safe_edge_paths {
        let first_edge = sequence.pop_front();
        let mut string_path = first_edge.unwrap().string;
        for edge in sequence {
            string_path += &edge.string[..k-1];
        }
        safe_paths.insert(string_path);
    }
    safe_paths
}


// Function that finds the longest safe path in a cycle starting from a certain node
fn find_longest_subwalk(sequence: &String, mut one_cycle: &mut VecDeque<Edge>, mut weight_left: &Weight, 
    former_weight: Weight, neighbor_weights: &mut Vec<Weight>, 
    safe_paths: &mut Vec<String>, mut safe_edge_paths: &mut Vec<VecDeque<Edge>>, i:usize, i2:usize, 
    edgelist: &Edgelist, cycle: &Vec<Edge>) -> (usize, Weight) {

        // ??????????????????
    // If there are no edges in our current path, reinitialize the variables
    if one_cycle.len() == 0 {
        weight_left = &0;
    }

    let seq;
    if sequence.len() == 0 {
        seq = String::from("");
        // weight_left = &0;
    } else {
        let range: usize = cycle[get_former_index(i, &cycle)].ending.len().clone();
        seq = sequence[range..].to_string();
    }

    // Our first pointer has moved, so we have to remove the first element of our path
    if !one_cycle.is_empty() {
        one_cycle.pop_front();
    }

    // Finding the longest path in our cycle starting with index i
    let (walk, index2, former_weight) = longest_subwalk(&cycle, i, i2, weight_left, former_weight, 
        neighbor_weights, seq, one_cycle, &edgelist);

    safe_paths.push(walk);
    safe_edge_paths.push(one_cycle.clone());
    (index2, former_weight)
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
    // let path = "../data/test_data/short.edgelist";
    // let path = "../data/test_data/sufpref.edgelist";
    // let path = "../data/test_data/outflow.edgelist";
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
    let cycles = build_cycles(edgelist.clone(), n_nodes);

    
    // Print the results
    print_cycles(&cycles);


    //---------------------------------------------------------------------------
    // Flow decomposition is done and the cycles are gathered.
    // Next, two-pointer algorithm.
    //---------------------------------------------------------------------------

    println!("************************************************************");

    let mut safe_paths = Vec::new();
    let mut safe_edge_paths = Vec::new();

    // Perform the algorithm on each cycle
    for cycle in cycles {

        // Initializing the vector for calculating paths in one cycle
        let mut one_cycle: VecDeque<Edge> = VecDeque::new();

        // If the cycle has only one edge, then the longest path in that cycle is that edge.
        if cycle.len() == 1 {
            safe_paths.push(cycle[0].string.clone());
            one_cycle.push_back(cycle[0].clone());
            safe_edge_paths.push(one_cycle);
        } else {
            // Setting up variables for a new cycle
            let mut i2 = 0; // Index of the second pointer
            let mut sequence = String::from("");
            let mut weight_left = 0; // The amount of flow left for the path to be safe
            let mut former_weight = 0; // The weight of the first edge of the path is stored, to be able to move the first pointer
            // let mut safety = true; // The variable is true as long as the path is safe
            let mut neighbor_weights = Vec::new(); // Vector containing the flow leaving outside of the cycle for eachnode in the cycle
            
            // Initializing the neighbor_weights-vector so that all the indexes can be accessed
            for _ in 0..(cycle.len()) {
                neighbor_weights.push(0);
            }

            // Calculating the safe paths for this cycle
            for i in 0..(cycle.len()) {
                (i2, former_weight) = find_longest_subwalk(&mut sequence, &mut one_cycle, &mut weight_left, 
                    former_weight, &mut neighbor_weights, &mut safe_paths, &mut safe_edge_paths, 
                    i, i2, &edgelist, &cycle);
            
            } 
        }

        // safe_edge_paths.push(one_cycle);

    }

    println!("\n+++++ Then, the safe paths as sequences: +++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        println!("Path {}:", counter);
        counter += 1;
        println!("{}", sequence);
    }

   
    println!("\n++++++++ Then, the safe paths as edges: ++++++++");
    let mut counter = 0;
    for sequence in &safe_edge_paths {
        println!("Path {}:", counter);
        for edge in sequence {
            print!("{} ", edge.string);
        }
        println!("");
        counter += 1;
    }


    let safe_paths = unique_sequences(safe_edge_paths, k);

    println!("\n++++++++ Then, the safe paths as final unique strings: ++++++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        println!("Path {}:", counter);
        println!("{} ", sequence);
        counter += 1;
    }

   
}

// Check maximality, then remove old sequence stuff
   

// cargo run -- '../data/test_data/outflow.edgelist'
