/*
 * File: H.rs
 * Project: BA3
 * File Created: 18th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */
/* Reconstruct a String from its k-mer Composition */

use std::collections::{BTreeMap, HashSet};
use super::G::{path_to_cycle, cycle_to_path};
use super::F::{find_cycle};

pub fn run(content: Vec<String>) {
    let k = content.get(0).unwrap().parse::<usize>().unwrap();

    // initialize kmers
    let mut kmer_vec: Vec<String> = Vec::new();
    let mut start_nodes: HashSet<usize> = HashSet::new();
    let mut from_to: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut node_degree: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut unexplored_edge = 0;
    for idx in 1..content.len() {
        kmer_vec.push(content[idx].clone());
        from_to.insert(idx-1, vec![]);
        node_degree.insert(idx-1, (0,0));
    }

    // find edges
    for i in 0..kmer_vec.len() {
        for j in i+1..kmer_vec.len() {
            if kmer_vec[i][0..k-1] == kmer_vec[j][1..k] { // j -> i
                from_to.entry(j).and_modify(|v| v.push(i));
                node_degree.entry(i).and_modify(|(in_degree,_)| *in_degree+=1);
                node_degree.entry(j).and_modify(|(_,out_degree)| *out_degree+=1);
                start_nodes.insert(j);
                unexplored_edge+=1;
            }

            if kmer_vec[i][1..k] == kmer_vec[j][0..k-1] { // i -> j
                from_to.entry(i).and_modify(|v| v.push(j));
                node_degree.entry(j).and_modify(|(in_degree,_)| *in_degree+=1);
                node_degree.entry(i).and_modify(|(_,out_degree)| *out_degree+=1);
                start_nodes.insert(i);
                unexplored_edge+=1;
            }
        }
    }

    let (p_start, p_end) = path_to_cycle(&mut from_to, &mut node_degree, &mut unexplored_edge);
    let cycle = find_cycle(&mut unexplored_edge, &mut from_to, &mut start_nodes, &mut node_degree);
    let path = cycle_to_path(cycle, p_start, p_end);

    let mut answer = String::new();
    for i in 0..path.len() {
        let kmer = kmer_vec.get(path[i]).unwrap();
        match i {
            0 => answer.push_str(kmer),
            _ => answer.push_str(&kmer[k-1..k])
        }
    }

    println!("{answer}");
}