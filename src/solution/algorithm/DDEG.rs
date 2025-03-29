/*
 * File: DDEG.rs
 * Project: algorithm
 * File Created: 29th Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Double-Degree Array */
fn id2idx(id: usize) -> usize {
    return id-1
}
struct Node {
    id: usize,
    degree: usize,
    neighbors: Vec<usize>
}

impl Node {
    fn new(id: usize) -> Node {
        Node {
            id:id,
            degree:0, neighbors: Vec::new()
        }
    }

    fn add_neighbor(&mut self, neighbor: usize) {
        self.degree+=1;
        self.neighbors.push(neighbor);
    }

    fn sum_neighbor_degrees(&self, nodes: &Vec<Node>) -> usize {
        let mut degrees = 0;
        let node_idx = id2idx(self.id);
        let neighbors = &nodes.get(node_idx).unwrap().neighbors;
        for neighbor in neighbors {
            let n_idx = id2idx(*neighbor);
            degrees += nodes.get(n_idx).unwrap().degree;
        }
        return degrees
    }
}

pub fn run(content: Vec<String>) {
    let node_edge: Vec<usize> = content.get(0).unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let mut it = node_edge.iter();
    let (n_node, n_edge) = (*it.next().unwrap(), *it.next().unwrap());

    let mut nodes: Vec<Node> = Vec::with_capacity(n_node);
    for i in 1..=n_node {
        nodes.push(Node::new(i));
    }
    
    for i in 1..content.len() {
        let n1_n2 : Vec<usize> = content.get(i).unwrap().split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let (n1, n2) = (*n1_n2.first().unwrap(), *n1_n2.last().unwrap());
        let (i1, i2) = (n1-1, n2-1);
        let mut node = nodes.get_mut(i1).unwrap();
        node.add_neighbor(n2);
        node = nodes.get_mut(i2).unwrap();
        node.add_neighbor(n1); 
    }

    for i in 0..nodes.len() {
        let node = nodes.get(i).unwrap();
        let d = node.sum_neighbor_degrees(&nodes);
        print!("{d} ");
    }
}