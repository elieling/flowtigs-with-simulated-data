// use std::env;
use std::fs;
mod edge;


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
    let n_nodes : i64 = n_nodes.parse().unwrap();

    let mut v: Vec<(i64, i64, i64, String,i64)> = Vec::new();
    let mut edgelist: Vec<Vec<(i64,i64,String,i64)>> = Vec::new();

    let mut empty : Vec<(i64,i64,String,i64)> = Vec::new();

    for i in 0..n_nodes {
        let mut ini = empty.clone();
        edgelist.push(ini);
    }

    let rounds = (&values).len() / 4;

    let mut id : i64 = 0;
    for i in 0..rounds {
        let n1: i64 = values[i*4+1].parse().unwrap();
        let n2: i64 = values[i*4+2].parse().unwrap();
        let n3: i64 = values[i*4+3].parse().unwrap();
        v.push((n1,n2,n3,(&values[i*4+4]).to_string(), id));

        edgelist[n1 as usize].push((n2,n3,(&values[i*4+4]).to_string(), id));
        id += 1;
    }

    println!("********************* {}, {} ***********************", &v.len(), rounds);
    for tup in &v {
        let (a, b, c, d, e) = tup;
        println!("From {} to {} with weight {}, string {} and id {}.", a, b, c, d, e);
    }

    println!("***********************************************");
    let mut counter = 0;
    // println!("0:");
    println!("Edgelist:");
    for vec in &edgelist {
        println!("{}:",counter);
        for tup in vec {
            let (a, b, c, d) = tup;
            print!("{} {} {} {} / ", a, b, c, d);
        }
        counter += 1;
        println!("");
    }
}



// Check flow condition