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
// use crate::cycle::ac_trie;
// use crate::cycle::try_removing;
mod ac_trie;
use crate::ac_trie::Trie;
use crate::ac_trie::build_trie;
use crate::ac_trie::insert_trie;









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
            let mut former_weight = 0;
            // let mut first 
            for i in 0..cycle.len() {
                let seq;
                if sequence.len() == 0 {
                    seq = String::from("");
                    weight_left = 0;
                    // former_weight = 0;
                }
                else {seq = sequence[1..].to_string();}
                let (walk, weight, index2, former_w) = longest_subwalk(&cycle, i, i2, weight_left, former_weight, seq, &edgelist);
                sequence = walk.clone();
                safe_paths.push(walk);
                weight_left = weight;
                former_weight = former_w;
                i2 = index2;
            }
        }
    }

    println!("\n+++++ Then, the safe paths: +++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        println!("Path {}:", counter);
        counter += 1;
        println!("{}", sequence);
    }

    // let filtered = ac_trie(&safe_paths);
    // // try_removing();
    // println!("\n+++++ Then, the safe paths after ac trie: +++++");
    // let mut counter = 0;
    // for sequence in filtered {
    //     println!("Path {}:", counter);
    //     counter += 1;
    //     println!("{}", sequence);
    // }


    let mut trie = build_trie();
    
    for sequence in &safe_paths {
        trie = insert_trie(trie, sequence.clone());
        println!("Sequence {}", sequence);
        println!("Length of trie is {}", &trie.nodes.len());
        println!("--------------------------------");
    }
    trie.print_trie();

}
    // FIX AHOCORASIK FUNCTION TO FIND SUFFIXES.