use crate::Edge;
use crate::edge::EdgeId;
use crate::edge::Weight;
use crate::graph::Edgelist;






    

pub fn get_former_index(index: usize, cycle: &Vec<Edge>) -> usize {
    if index != 0 {return(index-1);}
    cycle.len() - 1
}

fn get_next_index(index: usize, cycle: &Vec<Edge>) -> usize {
    if index == cycle.len() - 1 {return(0);}
    index+1
}


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
fn step (cycle: &Vec<Edge>, index: usize, weight: Weight, mut neighbor_weights: &mut Vec<Weight>, 
    edgelist: &Edgelist) -> (bool, Weight) {

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
    neighbor_weights[index] = neighbor_weight;
    let mut safety = false;
    if weight > 0 {safety = true;}
    else {println!("FALSE!");}
    (safety, weight)
}

fn check_neighbor_weight(cycle: &Vec<Edge>, index: usize, edgelist: &Edgelist) -> (Weight) {
    let edge = cycle[index].clone();
    let id = edge.id;
    let neighbors = &edgelist[edge.start_node];
    let mut neighbor_weight = 0;
    for (_, neigh) in neighbors {
        println!("Neighbor {} with weight {} of {} with weight {}", neigh.id, neigh.weight, edge.id, edge.weight);
        if neigh.id != id {
            neighbor_weight += neigh.weight;
        }
    }
    neighbor_weight
}



// Function that calculates the longest subwalk starting from a particular edge.
// Returns a String of the longest path starting from the node.
pub fn longest_subwalk(cycle: &Vec<Edge>, index1: EdgeId, index2: EdgeId, mut weight: &Weight, 
    mut former_weight: &Weight, mut neighbor_weights: &mut Vec<Weight>, sequence: String, 
    edgelist: &Edgelist)
 -> (String, EdgeId, bool) {
    let mut longest_path = sequence; 
    let index1 = index1;
    let mut index2 = index2;
    let original_edge = &cycle[index1];
    let last_edge_of_cycle = &cycle[get_former_index(index1, &cycle)];
    let mut safe = true;
    let mut weight_left = weight + original_edge.weight - former_weight + neighbor_weights[index1].clone();
    if longest_path.len() == 0 {
        longest_path += &original_edge.string;
        weight_left = original_edge.weight;
    }
    // println!("+++++ {} weight_left {} = weight {} + original_edge.weight {} - former_weight {}", &longest_path, weight_left, weight, original_edge.weight, former_weight);
    // let index_of_next_edge;
    // if index2 == cycle.len() - 1 {index_of_next_edge = 0;}
    // else {index_of_next_edge = index2 + 1;}
    // if weight_left <= check_neighbor_weight(&cycle, index2, &edgelist) {
    //     println!("Not enough flow left: {} <= {}. Indices are {} and {}.", weight_left, check_neighbor_weight(&cycle, index2, &edgelist), index1, index2);
    //     return (longest_path, weight_left, index2, original_edge.weight, neighbor_weights, false);
    // }
    // println!("Enough flow left: {}", weight_left);
    loop {

        // println!("Sequence {} with weight {} and indexes {} {}", longest_path, weight_left, index1, index2);
        let (edge, next_index) = next_edge(&cycle, index2);
        index2 = next_index;
        // println!("Sequence {} with weight {} and indexes {} {}", longest_path, weight_left, index1, index2);
        let (safety, weight) = step(&cycle, index2, weight_left, neighbor_weights, &edgelist); 
        // neighbor_weights = &neigh_weights;
        // println!("Safety {} Weight {}", safety, weight);
        if safety {
            // println!("- Sequence {} with ending {}", edge.string, edge.string[12..]);
            // longest_path.push(edge.last_char());
            println!("Longest_path {}", longest_path);
            longest_path += &edge.ending;
            weight_left = weight;
            // println!("get_next_index(edge.id {} , &cycle) {} == original_edge.id {}", edge.id, get_next_index(edge.id , &cycle), original_edge.id);
            // if edge.id == &original_edge.id {
            if edge.id == last_edge_of_cycle.id {
                println!("Got here!");
                // index2 = get_former_index(get_former_index(index2, &cycle), &cycle);
                break;
            }
        } else {
            if index2 == index1 && longest_path.len() < 2 {
                index2 = get_next_index(index2, &cycle);
            }
            else if index2 != get_next_index(index1, &cycle) {
                index2 = get_former_index(index2, &cycle);
            }
            safe = safety; break;
        }
        // index2 = next_index;
    }
    let original_weight = original_edge.weight.clone();
    former_weight &original_weight;
    (longest_path, index2, safe)
}


// cargo run -- '../data/test_data/outflow.edgelist'
