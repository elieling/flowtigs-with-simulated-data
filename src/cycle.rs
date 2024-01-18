use std::collections::VecDeque;
use crate::Edge;
use crate::edge::EdgeId;
use crate::edge::Weight;
// use crate::graph::Edgelist;






    

fn get_former_index(index: usize, cycle: &Vec<Edge>) -> usize {
    if index != 0 {return index-1;}
    cycle.len() - 1
}

fn get_next_index(index: usize, cycle: &Vec<Edge>) -> usize {
    if index == cycle.len() - 1 {return 0;}
    index+1
}


// Function to get the next edge in the cycle. 
// Returns the edge and its index in the cycle.
fn next_edge(cycle: &Vec<Edge>, index: usize) -> (Edge, usize) {
    let limit = cycle.len() - 1;
    if index == limit {
        return (cycle[0].clone(), 0);
    }
    (cycle[index+1].clone(), index+1)
}

// Function to test the safety when adding an edge to the path. 
// Returns whether it is safe and the remaining weight.  
fn step (index: usize, weight: Weight, neighbor_weights: &mut [Weight]) 
-> (bool, Weight) {
    let mut weight = weight;

    weight -= neighbor_weights[index];  
    // println!("weight {} -= neighbor_weights[index] {};  ", weight, neighbor_weights[index]);
    let mut safety = false;
    if weight > 0 {safety = true;}
    (safety, weight)
}




// Function that calculates the longest subwalk starting from a particular edge.
// Returns a String of the longest path starting from the node.
fn longest_subwalk(cycle: &Vec<Edge>, index1: EdgeId, index2: EdgeId, weight: Weight, 
    mut former_weight: Weight, neighbor_weights: &mut [Weight], one_cycle: &mut VecDeque<Edge>) 
    -> (EdgeId, Weight, Weight, Weight) {

    // let mut longest_path = sequence; 
    let mut index2 = index2;

    // We are storing the first edge of our path as well as the last possible potential edge
    let original_edge = &cycle[index1];
    let last_edge_of_cycle = &cycle[get_former_index(index1, cycle)];
    let mut extra_weight = original_edge.weight;



    // If the flow left is <= 0, the path is not safe
    let mut weight_left = weight + original_edge.weight - former_weight + neighbor_weights[index1];
    // println!("weight_left {} = weight {} + original_edge.weight {} - former_weight {} + neighbor_weights[index1].clone() {}", weight_left, weight, original_edge.weight, former_weight, neighbor_weights[index1]);

    // Keeping track of the weight of the first edge of the path
    former_weight = cycle[index1].weight;

    if one_cycle.is_empty() {
        one_cycle.push_back(original_edge.clone());
        weight_left = original_edge.weight;
    }

    // if longest_path.len() == 0 {
    //     longest_path += &original_edge.string;
    //     weight_left = original_edge.weight;
    // }
    
    // Making the path longer as long as it is safe 
    loop {
        let (edge, next_index) = next_edge(cycle, index2);
        index2 = next_index;
        let (safety, weight) = step(index2, weight_left, neighbor_weights); 
        if safety {
            // longest_path += &edge.ending;
            one_cycle.push_back(edge.clone());
            weight_left = weight;
            extra_weight = weight_left;
            //if edge.id == last_edge_of_cycle.id {
            //    break;
            //}
            if edge.id == index1 {
                if former_weight == weight_left {
                    index2 = get_former_index(index2, cycle);
                    one_cycle.pop_back();
                    break;
                }
            }
        } else {
            // Adjusting the indice for the the next round
            if index2 == index1 && one_cycle.len() < 2 {
                index2 = get_next_index(index2, cycle);
            }
            else if index2 != get_next_index(index1, cycle) {
                index2 = get_former_index(index2, cycle);
            }
            break;
        }
    }
    (index2, weight_left, former_weight, extra_weight)
}


 


// Function that finds the longest safe path in a cycle starting from a certain node
pub fn find_longest_subwalk(one_cycle: &mut VecDeque<Edge>, mut weight_left: Weight, 
    former_weight: Weight, neighbor_weights: &mut [Weight], safe_edge_paths: &mut Vec<VecDeque<Edge>>, 
    i:usize, i2:usize, cycle: &Vec<Edge>, extra_weight_of_paths: &mut Vec<Weight>) 
    -> (usize, Weight, Weight) {


    // If there are no edges in our current path, reinitialize the variables
    if one_cycle.is_empty() {
        weight_left = 0;
    }

    // let seq;
    // if sequence.len() == 0 {
    //     seq = String::from("");
    // } else {
    //     let range: usize = cycle[get_former_index(i, &cycle)].ending.len().clone();
    //     seq = sequence[range..].to_string();
    // }

    // Our first pointer has moved, so we have to remove the first element of our path
    if !one_cycle.is_empty() {
        one_cycle.pop_front();
    }


    // Finding the longest path in our cycle starting with index i
    let (index2, weight_left, former_weight, extra_weight) = longest_subwalk(cycle, i, i2, weight_left, 
        former_weight, neighbor_weights, one_cycle);

    // safe_paths.push(walk);
    safe_edge_paths.push(one_cycle.clone());
    extra_weight_of_paths.push(extra_weight);
    (index2, weight_left, former_weight)
}
    


// cargo run -- '../data/test_data/outflow.edgelist'