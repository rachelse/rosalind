/*
 * File: M.rs
 * Project: BA3
 * File Created: 9th Feb 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Generate All Maximal Non-Branching Paths in a Graph */

use super::K::{find_startnode, Node};

pub fn run(content: Vec<String>) {
    // Parse input and fill the nodes
    let mut node_vec: Vec<Node> = Vec::new();
    for line in content {
        let nodes: Vec<&str>  = line.split(" -> ").collect();
        let from = nodes.first().unwrap().parse::<usize>().unwrap();
        let tos : Vec<usize> = nodes.last().unwrap()
                                .split(',').into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
        
        if let None = node_vec.get(from) {
            for i in node_vec.len()..=from {
                let node = Node::new(i, "");
                node_vec.push(node);
            }
        }

        for to in &tos {
            if let None = node_vec.get(*to) {
                for i in node_vec.len()..=*to {
                    let node = Node::new(i, "");
                    node_vec.push(node);
                }
            }
        }

        let from_node = node_vec.get_mut(from).unwrap();
        for to in &tos {
            from_node.neighbors.push(*to);
            from_node.out_degree+=1;
        }
        for to in &tos {
            node_vec.iter_mut().nth(*to).map(|node| node.in_degree+=1);
        }
    }

    for i in 1..node_vec.len() {
        let node =  node_vec.get_mut(i).unwrap();
        if !((node.in_degree == 1) && (node.out_degree == 1)) {
            node.branch = true;
        }
    }

    let mut paths : Vec<Vec<usize>> = Vec::new();
    // Find all paths
    let mut start = find_startnode(&node_vec);
    while start != None {
        let mut curr_idx = start;
        let mut path: Vec<usize> = Vec::new();

        while curr_idx != None {
            let mut curr_node = node_vec.get_mut(curr_idx.unwrap()).unwrap();
            if (curr_idx!=start) {
                path.push(curr_idx.unwrap());
                curr_node.in_degree-=1;

                if (curr_node.branch==true) {
                    curr_idx = None;
                    break
                }
            } else {
                path.push(curr_idx.unwrap());
            }

            // Make sure the cycle not repeated
            if (curr_node.out_degree > 0) {
                curr_idx = curr_node.neighbors.pop();
                curr_node.out_degree-=1;
            } else {
                curr_idx = None;
            }
        }

        paths.push(path);
        start = find_startnode(&node_vec);
    }

    // Print the paths
    for path in paths {
        let mut path_string = String::new();
        for (i,p) in path.iter().enumerate() {
            if (i == 0) {
                path_string = format!("{p} ");
            } else {
                path_string.push_str(format!("-> {p} ").as_str());
            }
        }
        println!("{path_string}");
    }
}