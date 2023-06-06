use std::fs;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
mod edge;
use crate::edge::Edge;


type Node_id = usize;
type Edge_id = usize;
type Weight = i64;


fn build_edge(id: Edge_id, start_node: Node_id, end_node: Node_id, weight: Weight, string: String) -> Edge {
    Edge {
        id,
        start_node,
        end_node,
        weight,
        string,
    }
}

fn sort_tuple(tup1: &(Edge, Node_id), tup2: &(Edge, Node_id)) -> Ordering {
    let (a1, a2) = tup1;
    let (b1, b2) = tup2;
    if a2 > b2 {
        Ordering::Less
    } else if b2 > a2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}


fn main() {
    println!("Hello, world!");
    let path = "../data/short_k13.edgelist";

    println!("In file {}", path);

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let values: Vec<&str> = contents.split_whitespace().collect();

    let n_nodes = &values[0];
    let n_nodes : Node_id = n_nodes.parse().unwrap();

    let mut v: Vec<(usize, usize, i64, String,usize)> = Vec::new();
    let mut edgelist: Vec<HashMap<Edge_id, Edge>> = Vec::new();
    let mut edgelist2: Vec<Vec<Edge>> = Vec::new();

    let emptylist : Vec<Edge> = Vec::new();
    let empty : HashMap<Edge_id, Edge> = HashMap::new();

    // Check flow condition
    let mut indeg = vec![0; n_nodes];
    let mut outdeg = vec![0; n_nodes];


    for i in 0..n_nodes {
        let emp = emptylist.clone();
        edgelist2.push(emp);
        edgelist.push(empty.clone());
    }

    let rounds = (&values).len() / 4;

    let mut id : Edge_id = 0;
    for i in 0..rounds {
        let node1: Node_id = values[i*4+1].parse().unwrap();
        let node2: Node_id = values[i*4+2].parse().unwrap();
        let nodeweight: Weight = values[i*4+3].parse().unwrap();
        v.push((node1,node2,nodeweight,(&values[i*4+4]).to_string(), id));

        

        let e = build_edge(id, node1, node2, nodeweight, (&values[i*4+4]).to_string());
        edgelist2[node1 as usize].push(e.clone());
        edgelist[node1 as usize].insert(e.id, e);
        id += 1;

        // Check flow condition
        indeg[node1 as usize] += nodeweight;
        outdeg[node2 as usize] += nodeweight;
    }

    // Check flow condition
    let mut flow_condition = true;
    for i in 0..n_nodes {
        if indeg[i] != outdeg[i] {
            println!("PANIC WITH {}", i);
            flow_condition = false;
        }
    }
    if flow_condition {println!("Flow condition satisfied")}
    else {println!("ERROR: Flow condition noe satisfied")}

    println!("********************* {}, {} ***********************", &v.len(), rounds);
    for tup in &v {
        let (a, b, c, d, e) = tup;
        println!("From {} to {} with weight {}, string {} and id {}.", a, b, c, d, e);
    }

    

    println!("***********************************************");
    for node in &edgelist2 {
        for edge in node {
            println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
        }
    }

    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");
    for node in &edgelist {
        for (_, edge) in node {
            println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
        }
    }


    //---------------------------------------------------------------------------
    // Edgelist is created from file and flow condition is checked.
    //---------------------------------------------------------------------------


    let mut visited : HashSet<Node_id> = HashSet::new(); // visited.insert() .contains() .remove()
    let mut queue : VecDeque<Node_id> = VecDeque::new(); // 
    let mut cycles : Vec<Vec<Edge>> = Vec::new();

    // Put all the nodes in the queue
    for i in 0..n_nodes {
        queue.push_back(i);
    }

    while !queue.is_empty() {
        println!("While queue size: {}", queue.len());
        // println!("{}", queue.len());
        // let node = queue.pop_front();
        // let elist = edgelist2.clone();
        let node : Node_id = queue.pop_front().unwrap();
        if edgelist[node].is_empty() {
            continue;
        }
        if edgelist2[node].is_empty() {
            continue;
        }


        // Find a cycle and the minimum flow of that cycle.
        let mut edges = Vec::new();
        let keys: Vec<_> = edgelist[node].keys().collect();
        let mut min_flow2 = edgelist2[node][0].weight;
        let mut min_flow = edgelist[node][keys[0]].weight;
        let mut visited : HashSet<Node_id> = HashSet::new();
        let mut counter = 0;
        let mut new_node = node;
        let mut one_cycle : Vec<Edge> = Vec::new();
        // for key in &keys {
        //     println!("Key: {}", key);
        // }
        loop {
            // println!("Loop!");
            // println!("node {} != {} new_node", node, new_node);
            // println!("&edgelist2[new_node][counter].end_node {} != {} new_node", &edgelist2[new_node][counter].end_node, new_node);
            // while visited.contains(&edgelist2[new_node][counter].end_node) && &edgelist2[new_node][counter].end_node != &node {
            //     counter+=1;
            //     // println!("Counter = {}", counter);
            //     // println!("{} != {}", &edgelist2[new_node][counter].end_node, &node);
            // }
            let keys: Vec<_> = edgelist[new_node].keys().collect();
            while visited.contains(&edgelist[new_node][keys[counter]].end_node) && &edgelist[new_node][keys[counter]].end_node != &node {
                counter+=1;
            }
            // println!("Got through!");
            let edge2 = &edgelist2[new_node][counter];
            let edge = &edgelist[new_node][keys[counter]];
            one_cycle.push(edge.clone());
            // println!("From {} to {}", edge.start_node, edge.end_node);
            new_node = edge.end_node;
            visited.insert(new_node);
            edges.push(edge);
            min_flow2 = min(min_flow2, edge.weight);
            min_flow = min(min_flow, edge.weight);
            println!("Edge id: {}, edge weight: {}, min_flow: {} and min_flow2: {}", edge.id, edge.weight,min_flow, min_flow2);
            if new_node == node {
                break;
            } else {
                // println!("{} != {}", node, new_node);
            }
            counter = 0;
        }
        cycles.push(one_cycle.clone());
        println!("PUSH!");

        // Remove weight of the cycle from the graph
        let mut edges_to_remove = Vec::new();
        for edge in one_cycle {
            println!("Edge {} from {} to {} in one_cycle", {edge.id}, edge.start_node, edge.end_node);
            // edgelist2[edge.start_node][edge.end_node].weight -= min_flow2;
            if let Some(edge) = edgelist[edge.start_node].get_mut(&edge.id) {
                edge.weight -= min_flow;
            }
            // if edgelist2[edge.start_node][edge.end_node].weight == 0 {
            //     edges_to_remove.push((edge.clone(), edge.end_node));
            // }
            if edgelist[edge.start_node][&edge.id].weight == 0 {
                edges_to_remove.push((edge.clone(), edge.end_node));
            }
        }
        edges_to_remove.sort_by(sort_tuple);
        for tup in edges_to_remove {
            let (edge, _) = tup;
            // edgelist2[edge.start_node].swap_remove(edge.end_node);
            edgelist[edge.start_node].remove(&edge.id);
        }
        println!("*** Edgelist after removing ***");
        for node in &edgelist {
            for (_, edge) in node {
                println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
            }
        }
        queue.push_back(node);
    }

    println!("\n##### Next, the cycles: #####");
    let mut counter = 0;
    for cycle in cycles {
        println!("Cycle: {}", counter);
        counter += 1;
        for edge in cycle {
            println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string);
        }
    }




}

