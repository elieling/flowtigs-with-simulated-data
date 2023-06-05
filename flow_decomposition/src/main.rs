// use std::env;
use std::fs;
mod edge;
use crate::edge::Edge;

type Node_id = usize;
type Edge_id = usize;
type Weight = i64;


fn build_edge(id: i64, start_node: Node_id, end_node: Node_id, weight: i64, string: String) -> Edge {
    Edge {
        id,
        start_node,
        end_node,
        weight,
        string,
    }
}


fn main() {
    println!("Hello, world!");
    let path = "../data/short_k13.edgelist";
    // let path = "/pathtest.txt";

    println!("In file {}", path);

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let values: Vec<&str> = contents.split_whitespace().collect();

    let n_nodes = &values[0];
    let n_nodes : Node_id = n_nodes.parse().unwrap();

    let mut v: Vec<(usize, usize, i64, String,i64)> = Vec::new();
    let mut edgelist2: Vec<Vec<(i64,i64,String,i64)>> = Vec::new();
    let mut edgelist: Vec<Vec<Edge>> = Vec::new();

    let mut empty : Vec<(i64,i64,String,i64)> = Vec::new();
    let emptylist : Vec<Edge> = Vec::new();

    // Check flow condition
    let mut indeg = vec![0; n_nodes];
    let mut outdeg = vec![0; n_nodes];


    for i in 0..n_nodes {
        let ini = empty.clone();
        edgelist2.push(ini);
        let emp = emptylist.clone();
        edgelist.push(emp);
    }

    let rounds = (&values).len() / 4;

    let mut id : i64 = 0;
    for i in 0..rounds {
        let n1: Node_id = values[i*4+1].parse().unwrap();
        let n2: Node_id = values[i*4+2].parse().unwrap();
        let n3: i64 = values[i*4+3].parse().unwrap();
        v.push((n1,n2,n3,(&values[i*4+4]).to_string(), id));

        // edgelist2[n1 as usize].push((n2,n3,(&values[i*4+4]).to_string(), id));

        let e = build_edge(id, n1, n2, n3, (&values[i*4+4]).to_string());
        edgelist[n1 as usize].push(e);
        id += 1;

        // Check flow condition
        indeg[n1 as usize] += 1;
        outdeg[n2 as usize] += 1;
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
    let mut counter = 0;
    // println!("0:");
    println!("Edgelist:");
    for vec in &edgelist2 {
        println!("{}:",counter);
        for tup in vec {
            let (a, b, c, d) = tup;
            print!("{} {} {} {} / ", a, b, c, d);
        }
        counter += 1;
        println!("");
    }

    println!("***********************************************");
    for node in edgelist {
        for edge in node {
            println!("Edge {} from {} to {} with weight {} and sequence {}.", edge.id, edge.start_node, edge.end_node, edge.weight, edge.string)
        }
    }
}



// Check flow condition