/*
 * File: F.rs
 * Project: BA3
 * File Created: 4th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Find an Eulerian Cycle in a Graph */

use std::collections::BTreeMap;

fn select_next(node_degree: &BTreeMap<usize, (usize, usize)>, nexts : &Vec<usize>) -> (usize,usize) {
    let (mut idx, mut next) = (0,0);
    // println!("{nexts:?}");
    let mut max_degree = 0;
    for (i,n) in nexts.iter().enumerate() {
        let out_degree = node_degree.get(n).unwrap().1;
        if out_degree >= max_degree {
            idx = i;   
            max_degree = out_degree;
            next = *n;
        }
    }
    return (idx,next)
}

fn update_path(from: usize, to: usize, node_degree: &mut BTreeMap<usize, (usize, usize)>, path: &mut Vec<usize>) {
    // println!("{from}->{to}");
    // println!("{from}:{:?}", node_degree.entry(from));
    // println!("{to}:{:?}", node_degree.entry(to)); 
    path.push(to);
    node_degree.entry(from).and_modify(|(i, o)| *o-=1);
    node_degree.entry(to).and_modify(|(i, o)| *i-=1);
}

pub fn run(content: Vec<String>) {
    let mut from_to: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut node_degree: BTreeMap<usize, (usize,usize)> = BTreeMap::new();

    // Parse input lines and save to map
    for line in content {
        let nodes: Vec<&str>  = line.split(" -> ").collect();
        let from = nodes.first().unwrap().parse::<usize>().unwrap();
        let to : Vec<usize> = nodes.last().unwrap()
                                .split(',').into_iter().map(|x| x.parse::<usize>().unwrap()).collect();

        node_degree.entry(from).and_modify(|val| val.1+=to.len()).or_insert((0,to.len()));
        for val in &to {
            node_degree.entry(*val).and_modify(|val| val.0+=1).or_insert((1,0));
        }

        from_to.entry(from).or_insert(to);
    }

    let mut max_degree = node_degree.iter().max_by(|(node1,(in1,out1)), (node2, (in2, out2))| out1.cmp(&out2)).unwrap();
    let mut from = *max_degree.0;
    
    let mut path: Vec<usize> = vec![];
    &path.push(from);
    let mut nexts = from_to.get_mut(&from).unwrap();
    while nexts.len() > 0 {
        let (idx,to) = select_next(&node_degree, &nexts);
        update_path(from, to, &mut node_degree, &mut path);
        nexts.remove(idx);
        from = to;
        nexts = from_to.get_mut(&from).unwrap();
    }

    for (i,val) in path.iter().enumerate() {
        if (i==(path.len()-1)) {
            println!("{val}");
        } else {
            print!("{val}->");
        }
    }
}