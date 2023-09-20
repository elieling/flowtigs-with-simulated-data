use std::collections::HashSet;
use std::collections::VecDeque;
use crate::edge::Edge;
use crate::graph::build_graph;
use crate::flow::build_cycles;
use crate::flow::initialize_weight_of_neighbors_from;
use crate::cycle::find_longest_subwalk;
use crate::uniqueness::unique_sequences;
use crate::memory_meter::MemoryMeter;
use log::info;


pub fn safe_paths(path: &str, k: usize, mut meter: Option<&mut MemoryMeter>) -> HashSet<String> {

    
    

    // Read the data and build the graph
    let (edgelist, n_nodes, string_sequences) = build_graph(path, k);

    

    info!("Flow condition satisfied and data structure built successfully.");
    if let Some(ref mut meter) = meter {
        meter.report();
    }
    //---------------------------------------------------------------------------
    // Edgelist is created from file and flow condition is checked.
    // Next, flow decomposition algorithm.
    //---------------------------------------------------------------------------

    // Build a data structure containing all the cycles in the dbg
    let cycles = build_cycles(edgelist.clone(), n_nodes, &edgelist);

    info!("Cycles separated successfully.");
    if let Some(ref mut meter) = meter {
        meter.report();
    }

    // Count the number of edges in all cycles in total
    let mut n_edges = 0;
    for cycle in &cycles {
        n_edges += cycle.len();
    }

    info!("Cycles contain a total of {} edges", n_edges);

    // Check whether the graph contains separated components that are cycles.
    let limit: usize = 1;
    'outside_loop: for cycle in &cycles {
        for edge in cycle {
            if &edgelist[edge.start_node].keys().len() > &limit {continue 'outside_loop;}
        }
        // If a separated component is a cycle, it should have length 1.
        if cycle.len() > limit {info!("Found separated component of size {}", cycle.len())}
    }

    info!("Cycle components checked successfully.");
    if let Some(ref mut meter) = meter {
        meter.report();
    }

    
    // Print the results
    // print_cycles(&cycles);


    //---------------------------------------------------------------------------
    // Flow decomposition is done and the cycles are gathered.
    // Next, two-pointer algorithm.
    //---------------------------------------------------------------------------

    // The paths as edges
    let mut safe_edge_paths = Vec::new();
    // The extra weight left corresponding to each path
    let mut extra_weight_of_paths = Vec::new();
    // The weight of neighbors of each node for edges leaving from that node
    let weight_of_neighbors_of_each_node = initialize_weight_of_neighbors_from(&edgelist);

    // Perform the algorithm on each cycle
    for cycle in cycles {

        // Initializing the vector for calculating paths in one cycle
        let mut one_cycle: VecDeque<Edge> = VecDeque::new();

        // If the cycle has only one edge, then the longest path in that cycle is that edge.
        if cycle.len() == 1 {
            // safe_paths.push(cycle[0].string.clone());
            one_cycle.push_back(cycle[0].clone());
            safe_edge_paths.push(one_cycle);
            extra_weight_of_paths.push(cycle[0].weight);
        } else {
            // Setting up variables for a new cycle
            let mut i2 = 0; // Index of the second pointer
            let mut weight_left = 0; // The amount of flow left for the path to be safe
            let mut former_weight = 0; // The weight of the first edge of the path is stored, to be able to move the first pointer
            let mut neighbor_weights = Vec::new(); // Vector containing the flow leaving outside of the cycle for eachnode in the cycle
            
            // Initializing the neighbor_weights-vector 
            for edge in &cycle {
                let weight_from_same_node = weight_of_neighbors_of_each_node[edge.start_node];
                neighbor_weights.push(weight_from_same_node - edge.weight);
            }

            // Calculating the safe paths for this cycle
            for i in 0..(cycle.len()) {
                (i2, weight_left, former_weight) = find_longest_subwalk(&mut one_cycle, weight_left, 
                    former_weight, &mut neighbor_weights, &mut safe_edge_paths, 
                    i, i2, &cycle, &mut extra_weight_of_paths);
            
            } 
        }
    }

    info!("Safe paths calculated successfully.");
    if let Some(ref mut meter) = meter {
        meter.report();
    }


    let safe_paths = unique_sequences(safe_edge_paths, k, &extra_weight_of_paths, &edgelist, weight_of_neighbors_of_each_node, string_sequences);


    info!("Safe paths made to strings successfully.");
    if let Some(ref mut meter) = meter {
        meter.report();
    }

   safe_paths
}
