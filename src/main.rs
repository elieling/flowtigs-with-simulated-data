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
// use crate::graph::build_graph;
// use crate::graph::Edgelist;
mod flow;
// use crate::flow::build_cycles;
// use crate::flow::print_cycles;
// use crate::flow::initialize_weight_of_neighbors_from;
mod cycle;
// use crate::cycle::find_longest_subwalk;
mod uniqueness;
// use crate::uniqueness::is_maximal;
// use crate::uniqueness::unique_sequences;
// use crate::uniqueness::create_parent_structure;
mod safe_paths;
use crate::safe_paths::safe_paths;
// use crate::cycle::longest_subwalk;
// use crate::cycle::get_former_index;
// use crate::cycle::ac_trie;
// use crate::cycle::try_removing;
use log::info;
use simple_logger::SimpleLogger;
mod test;
// mod tests;

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
    SimpleLogger::new().init().unwrap();
    info!("Logging initialised successfully.");


    let safe_paths = safe_paths(path, k);


    println!("\n++++++++ Then, the safe paths as final unique strings: ++++++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        println!("Path {}:", counter);
        println!("{} ", sequence);
        counter += 1;
    }
}

