/* Implement the Neighbor Joining Algorithm */
use std::collections::BTreeMap;

fn _parse(line: &String) -> Vec<f32> {
    let parsed : Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
    parsed
}

fn computeJoinedDistance(D: &mut BTreeMap<usize,BTreeMap<usize,f32>>, n:usize) {
    let mut total_distances: BTreeMap<usize,f32> = BTreeMap::new();
    for (i, dij) in D.into_iter() {
        let dij_sum = dij.values().copied().reduce(|x,y| x+y).unwrap();
        total_distances.insert(*i, dij_sum);
    }
    let d_keys : Vec<usize> = D.keys().map(|x| x.to_owned()).collect();
    
    for i in 0..D.len() {
        for j in i+1..D.len() {
            let (ki, kj) = (&d_keys[i], &d_keys[j]);
            let dij = (n-2) as f32 * D[ki][kj] - total_distances[ki] - total_distances[kj];
            *D.get_mut(ki).unwrap().get_mut(kj).unwrap() = dij;
            *D.get_mut(kj).unwrap().get_mut(ki).unwrap() = dij; 
        }
    }
}

fn findClosest(D: &BTreeMap<usize,BTreeMap<usize,f32>>) -> (usize,usize) {
    let mut min_dist = f32::MAX;
    let (mut idx, mut jdx) = (0,0);
    for (i, map) in D {
        for (j, dist) in map {
            if min_dist > *dist {
                min_dist = *dist;
                (idx, jdx) = (*i, *j)
            }
        }
    }
    (idx,jdx)
}

fn removeColumn(D:&mut BTreeMap<usize,BTreeMap<usize,f32>>, i: usize, j: usize) {
    D.remove(&i);
    D.remove(&j);
    for (k, map) in D {
        map.remove(&i);
        map.remove(&j);
    }
}

fn connectLimbs(tree: &mut BTreeMap<usize,BTreeMap<usize,f32>>, i: usize, m: usize, limblength: f32) {
    let itree = tree.entry(i).or_insert(BTreeMap::new());
    itree.insert(m,limblength);
    let mtree = tree.entry(m).or_insert(BTreeMap::new());
    mtree.insert(i, limblength);
}

fn NeighborJoining(D: &mut BTreeMap<usize,BTreeMap<usize,f32>>, n: usize, new_node: usize) -> BTreeMap<usize, BTreeMap<usize, f32>> {
    if n == 2 {
        // let mut tree : BTreeMap<usize, BTreeMap<usize, f32>> = BTreeMap::new();
        // insertTree((0,1), D[&n-2][&n-1], &mut tree);
        let tree = D.clone();
        return tree
    }

    let mut D_joined = D.clone();
    computeJoinedDistance(&mut D_joined, n);

    let (i,j) = findClosest(&D_joined);
    let delta = (D[&i].values().copied().reduce(|x, y| (x+y)).unwrap() - D[&j].values().copied().reduce(|x, y| (x+y)).unwrap()) / (n-2) as f32;
    let limbi = (D[&i][&j] + delta) / 2.0;
    let limbj = (D[&i][&j] - delta) / 2.0;

    let mut new_map: BTreeMap<usize, f32> = BTreeMap::new();
    for (k, map) in D.iter() {
        if *k != i && *k!= j {
            let dmk = 0.5 * (map[&i] + map[&j] - D[&i][&j]);
            // D.get_mut(&k).unwrap().insert(new_node, dmk);
            new_map.insert(*k, dmk);
        }
    }
    removeColumn(D, i, j);
    for (i, val) in new_map.iter() {
        D.get_mut(i).unwrap().insert(new_node, *val);
    }
    D.insert(new_node, new_map);
    let mut tree = NeighborJoining(D, n-1, new_node+1);
    connectLimbs(&mut tree, i, new_node, limbi);
    connectLimbs(&mut tree, j, new_node, limbj);

    return tree
}

pub fn run(content: &Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    let mut D: BTreeMap<usize, BTreeMap<usize, f32>> = BTreeMap::new();

    for i in 1..=n {
        let parsed = _parse(&content[i]);
        D.insert(i-1, BTreeMap::new());
        for (j, val) in parsed.iter().enumerate() {
            if i-1 != j {
                D.get_mut(&(i-1)).unwrap().insert(j, *val);
            }
        }
    }
    let tree = NeighborJoining(&mut D, n, n);
    for (i,map) in tree {
        for (j, dist) in map {
            println!("{i}->{j}:{:.3}", dist);
        }
    }
}