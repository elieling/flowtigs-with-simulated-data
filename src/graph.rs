use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use log::error;
// use std::path::Path;
use std::collections::HashMap;
use crate::edge::Edge;
use crate::edge::build_edge;
use crate::edge::NodeId;
use crate::edge::EdgeId;
use crate::edge::Weight;
// use genome_graph::compact_genome::implementation::DefaultSequenceStore;
// use genome_graph::compact_genome::interface::alphabet::dna_alphabet::DnaAlphabet;
// use genome_graph::compact_genome::interface::sequence_store::SequenceStore;


pub type Edgelist = Vec<HashMap<EdgeId, Edge>>;

// /// A handle of a sequence in an [BitVectorSequenceStore].
// #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
// pub struct BitVectorSequenceStoreHandle<AlphabetType: Alphabet> {
//     offset: usize,
//     len: usize,
//     phantom_data: PhantomData<AlphabetType>,
// }

// type Handle = BitVectorSequenceStoreHandle<AlphabetType>;




// // Reading the file
// pub fn read_file(path: &str) -> String {
//     // println!("Using file {}", path);
//     fs::read_to_string(path)
//         .expect("Should have been able to read the file")
// }


// Creating data structure representing the graph and calculating indegree and outdegree of each node
fn create_graph(path: &str, n_nodes: NodeId, _k: usize) -> (Vec<HashMap<EdgeId, Edge>>, Vec<Weight>, Vec<Weight>) {
    
    // Setup empty data structure
    let mut edgelist: Vec<HashMap<EdgeId, Edge>> = Vec::new();
    let empty : HashMap<EdgeId, Edge> = HashMap::new();
    for _ in 0..n_nodes {
        edgelist.push(empty.clone());
    }

    // Setup for checking flow condition
    let mut indeg = vec![0; n_nodes];
    let mut outdeg = vec![0; n_nodes];

    // Create the graph
    // let rounds = (values).len() / 4;
    let mut id : EdgeId = 0;
    // for i in 0..rounds {
    // Variable to skip the first line of the input which contains only the number of nodes.
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
                    // assert_eq!(values.len(), 4, "Number of values on a line is expected to be 4, but was {}.", values.len());
                    let node1: NodeId = values[0].parse().unwrap();
                    let node2: NodeId = values[1].parse().unwrap();
                    let nodeweight: Weight = values[2].parse().unwrap();
                    // let ending = &values[i*4+4][(k-1)..].to_string();
                    let edge = build_edge(id, node1, node2, nodeweight); //, (&values[i*4+4]).to_string());
                    // The string sequence will be calculated later
                    edgelist[node1].insert(edge.id, edge);
                    id += 1;
            
                    // Counting indegree and outdegree for checking flow condition
                    indeg[node1] += nodeweight;
                    outdeg[node2] += nodeweight;
                } else {
                    error!("Error reading line");
                }
            }
        }
        Err(err) => {
            error!("Error opening file: {}", err);
        }
    }
    
    (edgelist, indeg, outdeg)
}


// Check whether the flow condition holds. If not, produces an error 
fn flow_condition(indeg: Vec<Weight>, outdeg: Vec<Weight>) {
    for i in 0..indeg.len() {
        assert_eq!(indeg[i], outdeg[i], "Flow condition not satisfied");
    }
}




// // Read the data and output the string sequences of each edge
// pub fn compute_string_sequences(path: &str) -> (DefaultSequenceStore<DnaAlphabet>, Vec<SequenceStore<DnaAlphabet>>) {

//     // Reading the file
//     let contents = read_file(path);
    
//     // Setup
//     let values: Vec<&str> = contents.split_whitespace().collect();

//     // Data structure to keep the string sequence related to an edge id
//     // let mut string_sequences: Vec<String> = Vec::new(); 
//     let mut sequence_store = DefaultSequenceStore::<DnaAlphabet>::new();

//     let mut handles = Vec::new();


//     // Create the graph
//     let rounds = (values).len() / 4;
//     for i in 0..rounds {
//         // string_sequences.push((&values[i*4+4]).to_string());
//         let handle = sequence_store.add_from_slice_u8((&values[i*4+4]).as_bytes());
//         handles.push(handle);
//     }

//     (sequence_store, handles)
//     // string_sequences
// }





// Read the data and build the graph
pub fn build_graph(path: &str, k: usize) -> (Vec<HashMap<EdgeId, Edge>>, NodeId) {

    // Reading the file
    // let contents = read_file(path);
    let mut n_nodes = 0;
    match File::open(path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(number) = line.trim().parse::<NodeId>() {
                        n_nodes = number;
                    } else {
                        error!("Error reading first line: {}", line);
                    }
                } else {
                    error!("Error reading line");
                }
                break;
            }
        }
        Err(err) => {
            error!("Error opening file: {}", err);
        }
    }

    // Setup
    // let values: Vec<&str> = contents.split_whitespace().collect();
    
    // let n_nodes = &values[0];
    // let n_nodes : NodeId = n_nodes.parse().unwrap();    

    // Creating data structure representing the graph 
    // let (edgelist, indeg, outdeg) = create_graph(values, n_nodes, k);
    let (edgelist, indeg, outdeg) = create_graph(path, n_nodes, k);

    // Check whether the flow condition holds. If not, produces an error 
    flow_condition(indeg, outdeg);
   
    (edgelist, n_nodes)
}


