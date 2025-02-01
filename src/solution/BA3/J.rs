/*
 * File: J.rs
 * Project: BA3
 * File Created: 26th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

 /* Reconstruct a String from its Paired Composition */

use std::collections::{BTreeMap, HashMap, HashSet};
use super::G::{path_to_cycle, cycle_to_path};
use super::F::find_cycle;

struct VirtualKmer {
    first: String,
    second: String
}

impl VirtualKmer {
    fn new(input: &String) -> VirtualKmer {
        let f_s: Vec<&str> = input.split("|").collect();
        VirtualKmer {
            first: f_s[0].to_string(),
            second: f_s[1].to_string(),
        }
    }

    fn prefix(&self) -> (String, String) {
        let k = self.first.len();
        let pref: (String, String) = (self.first[0..k-1].to_string(), self.second[0..k-1].to_string());
        return pref;
    }

    fn suffix(&self) -> (String, String) {
        let k = self.first.len();
        let suff: (String, String) = (self.first[1..k].to_string(), self.second[1..k].to_string());
        return suff;
    }

    fn toNode(&self, lastidx: &mut usize,
            node2idx: &mut HashMap<(String, String), usize>, nodevec: &mut Vec<(String, String)>
            ) -> (usize, usize) {
        let (mut n1, mut n2) = (0, 0);
        n1 = *lastidx;
        if let None = node2idx.get(&self.prefix()) {
            *lastidx+=1;
            node2idx.insert(self.prefix(), n1);
            nodevec.push(self.prefix());
        } else {
            n1 = *node2idx.get(&self.prefix()).unwrap();
        }
        n2 = *lastidx;
        if let None = node2idx.get(&self.suffix()) {
            *lastidx+=1;
            node2idx.insert(self.suffix(), n2);
            nodevec.push(self.suffix());
        } else {
            n2 = *node2idx.get(&self.suffix()).unwrap();
        }
        return (n1,n2);
    }
}

fn updateNodes(from_idx: usize, to_idx: usize,
    from_to: &mut BTreeMap<usize, Vec<usize>>, node_degree: &mut BTreeMap<usize, (usize, usize)>, start_nodes: &mut HashSet<usize>) 
{
    from_to.entry(from_idx).
            and_modify(|v| v.push(to_idx)).
            or_insert(vec![to_idx]);
    node_degree.entry(from_idx).
            and_modify(|(_, o)| *o+=1).
            or_insert((0,1));
    node_degree.entry(to_idx).
            and_modify(|(i, _)| *i+=1).
            or_insert((1,0)); 
    start_nodes.insert(from_idx);
}

pub fn run(content: Vec<String>) {
    let k_d : Vec<usize>= content.get(0).unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let (k, d) = (k_d[0], k_d[1]);

    let mut start_nodes: HashSet<usize> = HashSet::new();
    let mut from_to: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut node_degree: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut node2idx: HashMap<(String, String), usize> = HashMap::new();
    let mut nodevec: Vec<(String, String)> = Vec::new();

    let mut lastidx = 0;
    let mut unexplored_edge = 0;
    for i in 1..content.len() {
        let vkmer: VirtualKmer = VirtualKmer::new(&content[i]);
        let (from_idx, to_idx) = vkmer.toNode(&mut lastidx, &mut node2idx, &mut nodevec);
        updateNodes(from_idx, to_idx, &mut from_to, &mut node_degree, &mut start_nodes);
        unexplored_edge+=1;
    }

    let (p_start, p_end) = path_to_cycle(&mut from_to, &mut node_degree, &mut unexplored_edge);
    let cycle = find_cycle(&mut unexplored_edge, &mut from_to, &mut start_nodes, &mut node_degree);
    let path = cycle_to_path(cycle, p_start, p_end);

    // let vk_len = 2*k+d; // k+d+1
    let mut answer = String::new();
    for i in 0..path.len() {
        let node_idx = path.get(i).unwrap();
        let p1 = &nodevec.get(*node_idx).unwrap().0;
        match i {
            0 => answer.push_str(p1.as_str()),
            _ => answer.push(p1.chars().last().unwrap())
        }
    }
    for i in path.len()-(k+d)..path.len() {
        let node_idx = path.get(i).unwrap();
        let p2 = &nodevec.get(*node_idx).unwrap().1;
        answer.push(p2.chars().last().unwrap());
    }

    println!("{answer}");
}