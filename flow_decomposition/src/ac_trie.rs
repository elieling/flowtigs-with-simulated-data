// Module for building AC-trie and removing duplicate words as well as prefixes and suffixes.

use std::collections::HashMap;
use std::collections::VecDeque;



#[derive(Clone)]
pub struct Node {
    sequence: String,
    is_final: bool,
    children: HashMap<char, String>,
}

pub fn build_node(sequence:String) -> Node {
    Node {
        sequence,
        is_final: false,
        children: HashMap::new(),
    }
}

impl Node {
    pub fn make_final(&mut self) {
        self.is_final = true;
    }
}




pub struct Trie {
    pub root: String,
    pub nodes: HashMap<String, Node>,
}

pub fn build_trie() -> Trie {
    let node = build_node(String::from(""));
    let mut map = HashMap::new();
    map.insert(String::from(""), node);
    Trie {
        root: String::from(""),
        nodes: map,
    }
}

pub fn insert_trie(mut trie: Trie, string: String) -> Trie {
    println!("Length of trie is {}", &trie.nodes.len());
    let mut node = trie.root.clone();
    // let mut new_node = &self.root;
    let mut sequence = String::from("");
    // if let Some(x) = trie.nodes.get_mut(&sequence) {
    //     x.children.insert(string.chars().nth(0).unwrap(), string[0..1].to_string());
    // }
    // trie.nodes[&sequence].children.insert(string.chars().nth(0).unwrap(), string[0..1].to_string());
    for char in string.chars() {
        sequence.push(char);
        let new_node;
        if !trie.nodes[&node].children.contains_key(&char) {
            new_node = build_node(sequence.clone());
            trie.nodes.insert(sequence.clone(),new_node);
            
            if let Some(x) = trie.nodes.get_mut(&node) {
                x.children.insert(char, sequence.clone());
            }
            // trie.nodes[&node].children.insert(char, sequence.clone());
        } //else {
        //     new_node = self.nodes[&self.nodes[&node].children[&char]];
        // }
        node.push(char);
    }
    if let Some(x) = trie.nodes.get_mut(&sequence) {
        x.make_final();
    }
    // let mut final_node = &mut trie.nodes[&sequence];
    // final_node.is_final = true;
    // trie.nodes.insert(sequence, final_node.clone());
    // trie.nodes[&sequence].make_final(); //.is_final = true;
    trie
}


impl Trie {
    pub fn print_trie(&self) {
        // println!("PRINT!");
        let mut queue = VecDeque::new();
        // println!("Self root {}", &self.nodes[&self.root].children[&'A']);
        for (_, child) in &self.nodes[&self.root].children {
            // println!("print for");
            queue.push_back(child);
        }
        while !queue.is_empty() {
            // println!("print while");
            let nodeOpt = queue.pop_front();
            let node = nodeOpt.unwrap();

            // let dummy = build_node(node);
            // let finality;
            // if self.nodes[&node].is_final {finality = "is final";}
            // else {finality = "is not final";}
            println!("Node {}", &node); //self.nodes[&node].sequence); //, finality);

            for (_, child) in &self.nodes[node].children {
                queue.push_back(child);
            }
        }
    }
}