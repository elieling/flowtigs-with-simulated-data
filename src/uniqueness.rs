
use std::collections::HashSet;
use crate::graph::Edgelist;
use crate::edge::Edge;
// use crate::edge::NodeId;
use crate::edge::Weight;
use std::collections::VecDeque;
use std::cmp::max;




pub fn create_parent_structure(edgelist: &Edgelist) -> Vec<Vec<Edge>> {
    let mut parents = Vec::new();
    let empty_vector = Vec::new();
    for _ in 0..edgelist.len() {
        parents.push(empty_vector.clone());
    }
    // let mut counter = 0;
    for node in edgelist {
        for edge in node.values() {
            parents[edge.end_node].push(edge.clone());
        }
        // counter += 1;
    }
    parents
}



// Check if the safe path is maximal
pub fn is_maximal(path: &VecDeque<Edge>, edgelist: &Edgelist, weight_left: Weight, parents: &[Vec<Edge>], 
    weights_of_neighbors: &[Weight]) -> bool {

    let last_edge = path.back().unwrap();
    let first_edge = path.get(0).unwrap();

    // Right side
    let last_node = last_edge.end_node;
    let mut maximum_weight_of_a_neighbor = 0;
    let mut total_weight_of_neighbors = 0;
    for child in edgelist[last_node].values() {
        total_weight_of_neighbors += child.weight;
        if child.id == first_edge.id {continue;}
        maximum_weight_of_a_neighbor = max(maximum_weight_of_a_neighbor, child.weight);
    }
    // println!("weight_left {} > total_weight_of_neighbors {} - maximum_weight_of_a_neighbor {}", weight_left, total_weight_of_neighbors, maximum_weight_of_a_neighbor);
    
    // Check if the flow is sufficient to get a longer path. If yes, return false.
    if weight_left > total_weight_of_neighbors - maximum_weight_of_a_neighbor {
        return false;
    } 

    // Left side
    let first_node = first_edge.start_node;
    let mut maximum_weight_of_parent_edge = 0;
    for parent in &parents[first_node] {
        if parent.id != last_edge.id {
            maximum_weight_of_parent_edge = max(maximum_weight_of_parent_edge, parent.weight);
        }
    }
    // println!("weight_left {} + maximum_weight_of_parent_edge {} - weights_of_neighbors[first_node] {} > 0 {}", weight_left, maximum_weight_of_parent_edge, weights_of_neighbors[first_node], weight_left + maximum_weight_of_parent_edge - weights_of_neighbors[first_node]);

    // Check if the flow is sufficient to get a longer path. If yes, return false.
    if weight_left + maximum_weight_of_parent_edge - weights_of_neighbors[first_node] > 0 {
        return false;
    }

    true
}


fn reverse_byte(byte: u8) -> u8 {
    if byte == 65 {
        return 84;
    }
    if byte == 67 {
        return 71;
    }
    if byte == 71 {
        return 67;
    }
    65
}


fn get_smaller_between_iself_and_reverse_complement(sequence: String) -> String {
    let mut reverse_complement = String::from("");
    let mut counter = sequence.len();
    let byte_sequence = sequence.as_bytes();
    for _ in 0..sequence.len() {
        counter -= 1;
        // println!("{}, {}, {}, {}", counter, i, byte_sequence[counter], reverse_byte(byte_sequence[counter]));
        reverse_complement.push(reverse_byte(byte_sequence[counter]) as char);
    }
    // println!("{}", reverse_complement);
    if sequence < reverse_complement {
        return sequence;
    }
    reverse_complement
}



pub fn unique_sequences(safe_edge_paths: Vec<VecDeque<Edge>>, k: usize, weights: &[Weight], 
    edgelist: &Edgelist, weights_of_neighbors: Vec<Weight>) -> HashSet<String> {

    let parents = create_parent_structure(edgelist);
    let mut safe_paths = HashSet::new();
    let  mut counter = 0;
    for mut sequence in safe_edge_paths {
        if is_maximal(&sequence, edgelist, weights[counter], &parents, &weights_of_neighbors) {
            let first_edge = sequence.pop_front();
            let mut string_path = first_edge.unwrap().string;
            for edge in sequence {
                string_path += &edge.string[k-1..];
            }
            safe_paths.insert(get_smaller_between_iself_and_reverse_complement(string_path));
        }
        counter += 1;
    }
    // let a = get_smaller_between_iself_and_reverse_complement(String::from("ACACGGTT"));
    safe_paths
}
