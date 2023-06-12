use std::env::args;
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
// use crate::edge::Weight;
mod graph;
use crate::graph::build_graph;
// use crate::graph::Edgelist;
mod flow;
use crate::flow::build_cycles;
use crate::flow::print_cycles;
mod cycle;
use crate::cycle::longest_subwalk;









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

    // args
    let args: Vec<String> = args().collect();
    let path = &args[1];  // cargo run -- '../data/long_k27.edgelist'
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
            safe_paths.push(cycle[0].string.clone());
        } else {
            let mut i2 = 0;
            let mut sequence = String::from("");
            let mut weight_left = 0;
            for i in 0..cycle.len() {
                let seq;
                if sequence.len() == 0 {seq = String::from("");}
                else {seq = sequence[1..].to_string();}
                let (walk, weight, index2) = longest_subwalk(&cycle, i, i2, weight_left, seq, &edgelist);
                sequence = walk.clone();
                safe_paths.push(walk);
                weight_left = weight;
                i2 = index2;
            }
        }
    }

    println!("\n+++++ Then, the safe paths: +++++");
    let mut counter = 0;
    for sequence in safe_paths {
        println!("Path {}:", counter);
        counter += 1;
        println!("{}", sequence);
    }
}


// FIND OUT HOW TO IMPLEMENT SECOND POINTER.
// IMPLEMENT SO THAT SUBPATHS ARE NOT OUTPUT IN ADDITION TO LONGEST PATHS.
// MAYBE ADD A FUNCTION THAT UTILISES INFO LEFT FROM longest_subwalk().