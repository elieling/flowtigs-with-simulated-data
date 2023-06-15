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
mod ac_trie;
// use crate::ac_trie::Trie;
use crate::ac_trie::build_trie;
use crate::ac_trie::insert_trie;
use crate::ac_trie::find_leaves;




fn find_longest_subwalk(mut sequence: &String, mut weight_left: &Weight, mut former_weight: &Weight, 
    mut neighbor_weights: &mut Vec<Weight>, mut safe_paths: &mut Vec<String>, i:usize, i2:usize, 
    edgelist: &Edgelist, cycle: &Vec<Edge>) 
-> (usize, bool) {
    // println!("Sequence {}", sequence);
    // println!("cycle index {} {} sequence {} same {}", i, i2, &cycle[i].string, &cycle[get_former_index(i, &cycle)].string == &sequence);
    let seq;
    if sequence.len() == 0 {
        seq = String::from("");
        weight_left = &0;
        // former_weight = 0;
    }
    else {
        // println!("--- SEQUENCE {}", sequence);
        let range: usize = cycle[get_former_index(i, &cycle)].ending.len().clone();
        seq = sequence[range..].to_string();
        // println!("+++++ SEQ should start only similar {}", seq);
    }
    // println!("&cycle[i].ending.len() {}", &cycle[i].ending.len());
    // else {seq = sequence[1..].to_string();}
    let (walk, index2, safety) = longest_subwalk(&cycle, i, i2, weight_left, former_weight, 
        neighbor_weights, seq, &edgelist);
    // sequence = walk.clone();
    safe_paths.push(walk);
    // weight_left = weight;
    // former_weight = former_w;
    // i2 = index2;
    (index2, safety)
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

    // Perform the algorithm on each cycle
    for cycle in cycles {
        if cycle.len() == 1 {
            safe_paths.push(cycle[0].string.clone());
        } else {
            let mut i2 = 0;
            let mut sequence = String::from("");
            let mut weight_left = 0;
            let mut former_weight = 0;
            let mut safety = true;
            let mut neighbor_weights = Vec::new();
            // let mut first 
            for i in 0..(cycle.len()) {
                neighbor_weights.push(0);
            }
            for i in 0..(cycle.len()) {
                // neighbor_weights.push(0);
                // if !safety { //  && (i < i2) || i2 < i - 1
                //     println!("Not safe, destroying {}", sequence);
                //     sequence = String::from("");
                //     weight_left = 0;
                //     former_weight = 0;
                //     safety = true;
                //     // let former_index;
                //     // if (i2 == 0) {i2 = cycle.len() - 1;}
                //     // else {i2 -= 1;}
                //     // continue;
                // }
                // else {}
                (i2, safety) = 
                    find_longest_subwalk(&mut sequence, &mut weight_left, &mut former_weight, 
                        &mut neighbor_weights, &mut safe_paths, i, i2, &edgelist, &cycle);
            
            } // &mut 
        }
    }

    println!("\n+++++ Then, the safe paths: +++++");
    let mut counter = 0;
    for sequence in &safe_paths {
        println!("Path {}:", counter);
        counter += 1;
        println!("{}", sequence);
    }

   
   
    return();


    let mut trie = build_trie();
    
    for sequence in &safe_paths {
        trie = insert_trie(trie, sequence.clone());
        println!("Sequence {}", sequence);
        println!("Length of trie is {}", &trie.nodes.len());
        println!("--------------------------------");
    }
    trie.print_trie();

    let leaves = find_leaves(trie);

    println!("###########################################");
    println!("The leaves are:");
    for leaf in leaves {
        println!("{}", leaf);
    }


}
   

// cargo run -- '../data/test_data/outflow.edgelist'
