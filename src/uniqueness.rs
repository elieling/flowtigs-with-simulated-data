
// use std::collections::HashSet;
use crate::graph::Edgelist;
use crate::edge::Edge;
// use crate::edge::NodeId;
use crate::edge::Weight;
// use crate::graph::read_file;
use std::collections::VecDeque;
use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufWriter;
use std::io::Write;
use log::{info,error};
// use crate::memory_meter::MemoryMeter;
use genome_graph::compact_genome::implementation::DefaultSequenceStore;
use genome_graph::compact_genome::interface::alphabet::dna_alphabet::DnaAlphabet;
use genome_graph::compact_genome::interface::sequence_store::SequenceStore;
use genome_graph::bigraph::traitgraph::traitsequence::interface::Sequence;





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
    // println!("weight_left {} > total_wread_fileeight_of_neighbors {} - maximum_weight_of_a_neighbor {}", weight_left, total_weight_of_neighbors, maximum_weight_of_a_neighbor);
    
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


// fn read_sequence(edge: Edge, sequence_store: &Vec<DefaultSequenceStore<DnaAlphabet>>) -> String {
//     let mut string = String::new();
//     let sequence = &sequence_store.get(edge.id);
//     for character in sequence.iter() {
//         string += character.sequence_handle;
//     }
//     string
// }


pub fn unique_sequences(safe_edge_paths: Vec<VecDeque<Edge>>, k: usize, weights: &[Weight], 
    edgelist: &Edgelist, weights_of_neighbors: Vec<Weight>, path: &str, mut output: BufWriter<File>)  {

    // Reading the file
    // let contents = read_file(path);
    // Setup
    // let values: Vec<&str> = contents.split_whitespace().collect();
    // Data structure to keep the string sequence related to an edge id
    // let mut string_sequences: Vec<String> = Vec::new(); 
    let mut sequence_store = DefaultSequenceStore::<DnaAlphabet>::new();
    let mut handles = Vec::new();
    // Create the graph
    // let rounds = (values).len() / 4;
    // for i in 0..rounds {
    //     // string_sequences.push((&values[i*4+4]).to_string());
    //     let handle = sequence_store.add_from_slice_u8((&values[i*4+4]).as_bytes());
    //     handles.push(handle);
    // }
    let mut first_line = true;
    match File::open(path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if first_line {
                        first_line = false;
                        continue;
                    }
                    let values: Vec<&str> = line.split_whitespace().collect();
                    let handle = sequence_store.add_from_slice_u8((&values[3]).as_bytes());
                    handles.push(handle);
                } else {
                    error!("Error reading line");
                }
            }
        }
        Err(err) => {
            error!("Error opening file: {}", err);
        }
    }

    let parents = create_parent_structure(edgelist);
    // let mut safe_paths = HashSet::new();
    let  mut counter = 0;
    let  mut maximal_path_counter = 0;
    for mut sequence in safe_edge_paths {
        if is_maximal(&sequence, edgelist, weights[counter], &parents, &weights_of_neighbors) {
            let first_edge = sequence.pop_front().unwrap();
            let mut string = String::new();
            let new_sequence = &sequence_store.get(&handles[first_edge.id].clone().unwrap());
            for character in new_sequence.iter() {
                string += &character.to_string();
            }
            let mut string_path = string; // first_edge.unwrap().string;
            for edge in sequence {
                let mut string = String::new();
                let new_sequence = &sequence_store.get(&handles[edge.id].clone().unwrap());
                for character in new_sequence.iter() {
                    string += &character.to_string();
                }
                string_path += &string[k-1..]; // &edge.string[k-1..];
            }
            let string_sequence = get_smaller_between_iself_and_reverse_complement((&string_path).to_string());
            writeln!(output, ">Path_{}", maximal_path_counter).unwrap();
            writeln!(output, "{} ", string_sequence).unwrap();
            maximal_path_counter += 1;
        }
        counter += 1;
    }
    // let a = get_smaller_between_iself_and_reverse_complement(String::from("ACACGGTT"));
    // safe_paths

    // // println!("\n++++++++ Then, the safe paths as final unique strings: ++++++++");
    // let mut counter = 0;
    // for sequence in &safe_paths {
    //     // println!("Path {}:", counter);
    //     // println!("{} ", sequence);
    //     counter += 1;
    // }
    info!("Safe paths written to file");
    // meter.report();
}


