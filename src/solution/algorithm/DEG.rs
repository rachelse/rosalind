/*
 * File: DEG.rs
 * Project: algorithm
 * File Created: 22nd Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

pub fn run(content:Vec<String>) {
    let node_edge: Vec<usize> = content.get(0).unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let mut it = node_edge.iter();
    let (node, edge) = (*it.next().unwrap(), *it.next().unwrap());

    let edges: Vec<(usize, usize)> = content.iter().skip(1)
                    .map(|x| x.split_whitespace().into_iter().map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>())
                    .map(|v| (v[0], v[1])).collect();

    let mut degree = vec![0;node];

    for idx in 0..edge {
        let (i,o) = edges.get(idx).unwrap();
        *degree.get_mut(*i-1).unwrap()+=1;
        *degree.get_mut(*o-1).unwrap()+=1;
    }

    for d in degree {
        print!("{d} ");
    }
}