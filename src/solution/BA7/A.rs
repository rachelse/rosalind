/* Compute Distances Between Leaves */

use std::collections::BTreeMap;

fn _parse(line:&String) -> (usize, usize, usize) {
    let parsed : Vec<&str> = line.split(":").collect();
    let nodes : Vec<usize> = parsed[0].split("->").map(|x| x.parse::<usize>().unwrap()).collect();
    let weight = parsed[1].parse::<usize>().unwrap();
    (nodes[0], nodes[1], weight)
}

fn compute_distances(n:usize, D:&mut BTreeMap<usize,BTreeMap<usize,usize>>) -> Vec<Vec<usize>> {
    let mut matrix : Vec<Vec<usize>> = vec![vec![0;n];n];

    for i in 0..n {
        let mut visited: Vec<bool> = vec![false;D.len()];
        let mut candidates: Vec<usize> = Vec::new();
        let mut distances : Vec<usize> = vec![0;D.len()];
        candidates.push(i);
        for j in 0..=i {
            visited[j] = true;
        }
    
        while candidates.len() > 0 {
            let c = candidates.remove(0);
            for n in D[&c].keys() {
                if !visited[*n] {
                    candidates.push(*n);
                    distances[*n] = distances[c] + D[&c][n];
                    visited[*n] = true;
                }
            }
        }

        for j in i+1..n {
            matrix[i][j] = distances[j];
            matrix[j][i] = distances[j];
        }
    }
    matrix
}

pub fn run(content:&Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    
    let mut D : BTreeMap<usize,BTreeMap<usize,usize>> = BTreeMap::new();
    for i in 1..content.len() {
        let (n1,n2,w) = _parse(&content[i]);
        let mut out = D.entry(n1).or_insert(BTreeMap::new());
        out.insert(n2,w);
    }

    let matrix = compute_distances(n, &mut D);

    for line in matrix {
        for element in line {
            print!("{element} ");
        }
        println!();
    }
}