use crate::Edge;
use crate::edge::EdgeId;
use crate::edge::Weight;
use crate::graph::Edgelist;



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
    let edge = cycle[index].clone();
    let id = edge.id;
    let neighbors = &edgelist[edge.start_node];
    let mut weight = weight;

    let mut neighbor_weight = 0;
    for (_, neigh) in neighbors {
        if neigh.id != id {
            neighbor_weight += neigh.weight;
        }
    }

    weight -= neighbor_weight;
    let mut safety = false;
    if weight > 0 {safety = true};
    (safety, weight)
}


// fn longest_subwalk(cycle: &Vec<Edge>, index: usize, edgelist: &Edgelist) -> (String, Weight) {
//     let mut longest_path = String::from("");
//     let mut index = index;
//     let original_edge = &cycle[index];
//     longest_path += &original_edge.string;
//     let mut weight_left = original_edge.weight;
//     loop {
//         let (edge, next_index) = next_edge(&cycle, index);
//         index = next_index;
//         let (safety, weight) = step(&cycle, index, weight_left, &edgelist); 
//         // println!("weight: {}", weight);
//         if safety {
//             longest_path.push(edge.last_char());
//             weight_left = weight;
//             if edge.id == original_edge.id {break;}
//         } else {break;}
//     }
//     (longest_path, weight_left)
// }

// Function that calculates the longest subwalk starting from a particular edge.
// Returns a String of the longest path starting from the node.
pub fn longest_subwalk(cycle: &Vec<Edge>, index1: EdgeId, index2: EdgeId, weight: Weight, sequence: String, edgelist: &Edgelist) -> (String, Weight, EdgeId) {
    let mut longest_path = sequence; //String::from(""); String::from("");
    let index1 = index1;
    let mut index2 = index2;
    let original_edge = &cycle[index1];
    let mut weight_left = weight + original_edge.weight;
    if longest_path.len() == 0 {
        longest_path += &original_edge.string;
        weight_left = original_edge.weight;
    }
    loop {
        let (edge, next_index) = next_edge(&cycle, index2);
        // println!("Edge {} with char {}", edge.string, edge.first_char());
        println!("Sequence {} with weight {} and indexes {} {}", longest_path, weight_left, index1, index2);
        index2 = next_index;
        let (safety, weight) = step(&cycle, index2, weight_left, &edgelist); 
        // println!("weight: {}", weight);
        if safety {
            longest_path.push(edge.last_char());
            weight_left = weight;
            if edge.id == original_edge.id {break;}
        } else {break;}
    }
    (longest_path, weight_left, index2)
}
