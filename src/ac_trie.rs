// Module for building AC-trie and removing duplicate words as well as prefixes and suffixes.

use std::collections::HashMap;
use std::collections::HashSet;
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
    let mut sequence = String::from("");

    for char in string.chars() {
        sequence.push(char);
        let new_node;
        if !trie.nodes[&node].children.contains_key(&char) {
            new_node = build_node(sequence.clone());
            trie.nodes.insert(sequence.clone(),new_node);
            
            if let Some(x) = trie.nodes.get_mut(&node) {
                x.children.insert(char, sequence.clone());
            }
        } 
        node.push(char);
    }
    if let Some(x) = trie.nodes.get_mut(&sequence) {
        x.make_final();
    }
    
    trie
}

pub fn find_leaves(trie: Trie) -> HashSet<String> {
    let mut queue = VecDeque::new();
    let mut leaves = HashSet::new();

    for (_, child) in &trie.nodes[&trie.root].children {
        queue.push_back(child);
    }
    while !queue.is_empty() {
        let node_opt = queue.pop_front();
        let node = node_opt.unwrap();

        if trie.nodes[node].children.is_empty() {
            leaves.insert(node.clone());
        } else {
            for (_, child) in &trie.nodes[node].children {
                queue.push_back(child);
            }
        }
    }
    leaves
}


impl Trie {
    pub fn print_trie(&self) {
        let mut queue = VecDeque::new();
        for (_, child) in &self.nodes[&self.root].children {
            queue.push_back(child);
        }
        while !queue.is_empty() {
            let node_opt = queue.pop_front();
            let node = node_opt.unwrap();

            println!("Node {}", &node); 

            for (_, child) in &self.nodes[node].children {
                queue.push_back(child);
            }
        }
    }
}