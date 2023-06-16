
use std::collections::HashSet;
use crate::graph::Edgelist;
use crate::edge::Edge;
use crate::edge::Weight;
use std::collections::VecDeque;
use std::cmp::max;



// Check if the safe path is maximal
pub fn is_maximal(path: VecDeque<Edge>, edgelist: Edgelist, weight_left: Weight) -> bool {

    // Right side
    let last_edge = path.get(path.len()-1).unwrap();
    let first_edge = path.get(0).unwrap();
    let last_node = last_edge.end_node;
    let mut maximum_weight_of_a_neighbor = 0;
    let mut total_weight_of_neighbors = 0;
    for (_, child) in &edgelist[last_node] {
        total_weight_of_neighbors += child.weight;
        if child.id == first_edge.id {continue;}
        maximum_weight_of_a_neighbor = max(maximum_weight_of_a_neighbor, child.weight);
    }
    println!("weight_left {} > total_weight_of_neighbors {} - maximum_weight_of_a_neighbor {}", weight_left, total_weight_of_neighbors, maximum_weight_of_a_neighbor);
    // Check if the flow is sufficient to get a longer path. If yes, return false.
    if weight_left > total_weight_of_neighbors - maximum_weight_of_a_neighbor {
        return false;
    } 

    true
}



pub fn unique_sequences(safe_edge_paths: Vec<VecDeque<Edge>>, k: usize) -> HashSet<String> {
    let mut safe_paths = HashSet::new();
    for mut sequence in safe_edge_paths {
        let first_edge = sequence.pop_front();
        let mut string_path = first_edge.unwrap().string;
        for edge in sequence {
            string_path += &edge.string[k-1..];
        }
        safe_paths.insert(string_path);
    }
    safe_paths
}
