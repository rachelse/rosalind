/*
 * File: F.rs
 * Project: BA3
 * File Created: 4th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Find an Eulerian Cycle in a Graph */

use std::collections::{BTreeMap,HashSet};

pub fn parse_input(content: &Vec<String>, 
    from_to : &mut BTreeMap<usize, Vec<usize>>, 
    node_degree: &mut BTreeMap<usize, (usize, usize)>,
    unexplored_edge : &mut usize, start_nodes: &mut HashSet<usize>
    ) {
    
    for line in content {
        let nodes: Vec<&str>  = line.split(" -> ").collect();
        let from = nodes.first().unwrap().parse::<usize>().unwrap();
        let to : Vec<usize> = nodes.last().unwrap()
                                .split(',').into_iter().map(|x| x.parse::<usize>().unwrap()).collect();

        node_degree.entry(from).and_modify(|val| val.1+=to.len()).or_insert((0,to.len()));
        for val in &to {
            node_degree.entry(*val).and_modify(|val| val.0+=1).or_insert((1,0));
            *unexplored_edge+=1;
        }

        from_to.entry(from).or_insert(to);
        start_nodes.insert(from);
    }
}

fn select_start(start_nodes: &mut HashSet<usize>, node_degree: &BTreeMap<usize, (usize, usize)>, cycle: &Vec<usize>) -> usize {
    // remove consumed nodes from start_nodes
    let mut to_remove = Vec::new();
    for i in start_nodes.iter() {
        if let Some((0,0)) = node_degree.get(i) {
            to_remove.push(*i);
        } else {continue;}
    }
    for i in to_remove {
        start_nodes.remove(&i);
    }

    // pick next start node among the available nodes
    for i in cycle.iter() {
        if let Some(_) = start_nodes.get(i) {
            return *i
            // start = *i;
            // break;
        } else {continue;}
    }

    return 0
}

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

fn update_path(from: usize, to: usize, node_degree: &mut BTreeMap<usize, (usize, usize)>, cycle: &mut Vec<usize>) {
    cycle.push(to);
    node_degree.entry(from).and_modify(|(i, o)| *o-=1);
    node_degree.entry(to).and_modify(|(i, o)| *i-=1);
}

pub fn print_path(path: &Vec<usize>) {
    for (i,val) in path.iter().enumerate() {
        if (i==(path.len()-1)) {
            println!("{val}");
        } else {
            print!("{val}->");
        }
    }
}

pub fn find_cycle( unexplored_edge : &mut usize, 
    from_to : &mut BTreeMap<usize, Vec<usize>>, start_nodes: &mut HashSet<usize>, 
    node_degree: &mut BTreeMap<usize, (usize, usize)>,
    ) -> Vec<usize> {
    let mut cycle: Vec<usize> = vec![];
    while *unexplored_edge > 0 {
        /* select newStart */
        let mut _cycle: Vec<usize> = Vec::new();

        let mut start;
        
        if cycle.len() == 0 {
            start = *start_nodes.iter().nth(0).unwrap();

        } else {
            /* select another start */
            start = select_start(start_nodes, &node_degree, &cycle);

            /* form Cycle' by traversing Cycle */
            let idx = cycle.iter().enumerate().find(|(idx ,&val)| val == start).unwrap().0;
            for i in idx..cycle.len() {
                _cycle.push(*cycle.get(i).unwrap());
            }
            for i in 1..idx {
                _cycle.push(*cycle.get(i).unwrap())
            }
        }
        
        _cycle.push(start);
        let mut nexts= from_to.get_mut(&start).unwrap();

        /* random walking */
        while nexts.len() > 0 {
            let (idx, next) = select_next(&node_degree, nexts);
            update_path(start, next, node_degree, &mut _cycle);
            nexts.remove(idx);
            *unexplored_edge -= 1;

            start = next;
            let _nexts = from_to.get_mut(&start);
            nexts = from_to.get_mut(&start).unwrap()   ;
        }
        start_nodes.remove(&start); 

        cycle = _cycle;
    }
    cycle
}

pub fn run(content: Vec<String>) {
    let mut from_to: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut node_degree: BTreeMap<usize, (usize,usize)> = BTreeMap::new();
    let mut unexplored_edge: usize = 0;
    let mut start_nodes: HashSet<usize> = HashSet::new();

    // Parse input lines and save to map
    parse_input(&content, &mut from_to, &mut node_degree, &mut unexplored_edge, &mut start_nodes);

    let cycle = find_cycle(&mut unexplored_edge, &mut from_to, &mut start_nodes, &mut node_degree);

    print_path(&cycle);
}