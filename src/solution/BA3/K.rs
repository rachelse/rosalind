/*
 * File: K.rs
 * Project: BA3
 * File Created: 31st Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Generate Contigs from a Collection of Reads */

use std::collections::BTreeMap;
use crate::print_answer;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub pattern: String,
    pub in_degree: usize,
    pub out_degree: usize,
    pub neighbors: Vec<usize>,
    pub branch: bool
}

impl Node {
    pub fn new(id: usize, pattern: &str) -> Node {
        Node {
            id: id, pattern: pattern.to_string(),
            in_degree: 0, out_degree: 0,
            neighbors: vec![], branch: false
        }
    }

}

pub fn find_startnode(nodes: &Vec<Node>) -> Option<usize> {
    let mut start: Option<usize> = None;
    for node in nodes {
        if (node.in_degree == 0 && node.out_degree > 0) {
            start = Some(node.id);
            return start
        } else if (node.in_degree !=0 && node.out_degree != 0) {
            start = Some(node.id); 
        }
    }
    return start
}

pub fn run(content: Vec<String>) {
    let k = content[0].len();

    // Parse input and fill the nodes
    let mut node_id: BTreeMap<String, usize> = BTreeMap::new();
    let mut node_vec: Vec<Node> = Vec::new();
    let mut node_counter = 0;
    for i in 0..content.len() {
        let from = &content.get(i).unwrap()[..k-1];
        let to = &content.get(i).unwrap()[1..];
        let (mut from_idx, mut to_idx) = (0, 0);
        if let None = node_id.get(from) {
            node_id.insert(from.to_string(), node_counter);
            from_idx = node_counter;
            node_counter+=1;
            let from_node = Node::new(from_idx, from);
            node_vec.push(from_node);
        } else {
            from_idx = *node_id.get(from).unwrap();
            
        }
        
        if let None = node_id.get(to) {
            node_id.insert(to.to_string(), node_counter);
            to_idx = node_counter;
            node_counter+=1;
            let to_node = Node::new(to_idx, to);
            node_vec.push(to_node);
        } else {
            to_idx = *node_id.get(to).unwrap();
        } 

        let from_node = node_vec.get_mut(from_idx).unwrap();
        from_node.neighbors.push(to_idx);
        from_node.out_degree+=1;
        let to_node = node_vec.get_mut(to_idx).unwrap();
        to_node.in_degree+=1;
    }
    
    // identify if they are branching or non-branching
    for i in 0..node_counter {
        let node = node_vec.get_mut(i).unwrap();
        if (node.in_degree != 1) || (node.out_degree != 1) {
            node.branch = true;
        }
    }

    let mut contigs: Vec<String> = Vec::new();
    
    let mut start = find_startnode(&node_vec);
    while start != None {
        let mut curr_idx = start;
        let mut contig = String::new();
        
        while curr_idx != None {
            let mut curr_node = node_vec.get_mut(curr_idx.unwrap()).unwrap();
            if (curr_idx != start) {
                contig.push(curr_node.pattern.chars().last().unwrap());
                curr_node.in_degree -=1;
                if (curr_node.branch == true) {
                    curr_idx = None;
                    break
                }
            } else {
                contig.push_str(&curr_node.pattern);
            }
            curr_idx = curr_node.neighbors.pop();
            curr_node.out_degree -=1;
        }

        contigs.push(contig);
        start = find_startnode(&node_vec);
    }

    print_answer(contigs);
}