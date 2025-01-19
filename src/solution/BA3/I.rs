/*
 * File: I.rs
 * Project: BA3
 * File Created: 18th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Find a k-Universal Circular String */
use std::collections::{BTreeMap, HashSet};
use super::F::{find_cycle};

fn make_binstring(k: usize, binString: String, kmerVec : &mut Vec<String>) {
    if (binString.len() == k) {
        kmerVec.push(binString);
    } else {
        let bin0= String::from(format!("{binString}0"));
        let bin1= String::from(format!("{binString}1"));
        make_binstring(k, bin0, kmerVec);
        make_binstring(k, bin1, kmerVec);
    }
}

pub fn run(content: Vec<String>) {
    let k = content[0].parse::<usize>().unwrap() -1;
    
    // generate kmers
    let mut kmerVec: Vec<String> = Vec::new();
    make_binstring(k, String::new(), &mut kmerVec);
    
    let mut start_nodes: HashSet<usize> = HashSet::new();
    let mut from_to: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut node_degree: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut unexplored_edge = 0;
    for idx in 0..kmerVec.len() {
        from_to.insert(idx, vec![]);
        node_degree.insert(idx, (0,0));
    }
    
    // find edges
    for i in 0..kmerVec.len() {
        for j in i..kmerVec.len() {
            if kmerVec[i][0..k-1] == kmerVec[j][1..k] { // j -> i
                from_to.entry(j).and_modify(|v| v.push(i));
                node_degree.entry(i).and_modify(|(in_degree,_)| *in_degree+=1);
                node_degree.entry(j).and_modify(|(_,out_degree)| *out_degree+=1);
                start_nodes.insert(j);
                unexplored_edge+=1;
            } 
            if (i!=j && kmerVec[i][1..k] == kmerVec[j][0..k-1]) { // i -> j
                from_to.entry(i).and_modify(|v| v.push(j));
                node_degree.entry(j).and_modify(|(in_degree,_)| *in_degree+=1);
                node_degree.entry(i).and_modify(|(_,out_degree)| *out_degree+=1);
                start_nodes.insert(i);
                unexplored_edge+=1;
            }
        }
    }

    let cycle = find_cycle(&mut unexplored_edge, &mut from_to, &mut start_nodes, &mut node_degree);

    let mut answer = String::new();
    for i in 0..&cycle.len()-k {
        let kmer = kmerVec.get(cycle[i]).unwrap();
        match i {
            0 => answer.push_str(kmer),
            _ => answer.push_str(&kmer[k-1..k])
        }
    }

    println!("{}",answer);
}