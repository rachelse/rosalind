/*
 * File: G.rs
 * Project: BA3
 * File Created: 11th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

use std::collections::{BTreeMap, HashSet};
use super::F::{parse_input, find_cycle, print_path};

pub fn run(content: Vec<String>) {
    let mut from_to: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut node_degree: BTreeMap<usize, (usize,usize)> = BTreeMap::new();
    let mut unexplored_edge: usize = 0;
    let mut start_nodes: HashSet<usize> = HashSet::new();

    parse_input(&content, &mut from_to, &mut node_degree, &mut unexplored_edge, &mut start_nodes);

    /* transform eulerian path to cycle */
    let mut p_start=0;
    let mut p_end = 0;

    for (node, (d_in, d_out)) in &node_degree {
        if d_in < d_out {
            p_start = *node;
        } else if d_in > d_out {
            p_end = *node;
        }
    }
    from_to.entry(p_end).and_modify(|nexts| nexts.push(p_start)).or_insert(vec![p_start]);
    node_degree.entry(p_end).and_modify(|degree| degree.1+=1);
    node_degree.entry(p_start).and_modify(|degree| degree.0+=1); 
    unexplored_edge+=1;

    /* construct eulerian cycle */
    let cycle = find_cycle(&mut unexplored_edge, &mut from_to, &mut start_nodes, &mut node_degree);

    /* transform cycle to path: remove p_end -> p_start */
    let mut i_end= 0 ;
    for i in 0..cycle.len()-1 {
        if (*cycle.get(i).unwrap() == p_end) && (*cycle.get(i+1).unwrap()==p_start) {
            i_end = i;
        }
    }

    let mut path = Vec::new();
    for i in i_end+1..cycle.len() {
        path.push(cycle[i]);
    }
    for i in 1..=i_end {
        path.push(cycle[i])
    }

    print_path(&path);
}