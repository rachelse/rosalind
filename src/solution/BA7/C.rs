/* Implement AdditivePhylogeny */
use std::collections::BTreeMap;
fn _parse(line: &String) -> Vec<usize> {
    let parsed : Vec<usize> = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    parsed
}

fn computeLimb(n:usize, D:&Vec<Vec<usize>>) -> usize {
    let mut i_k : Vec<(usize,usize)> = Vec::new();
    for i in 0..D.len() {
        if i == n-1 {continue}
        for k in i+1..D.len() {
            if k == n-1 {continue}
            i_k.push((i,k));
        }
    }
    let mut min_limblength = usize::MAX;
    let (mut mini,mut mink) = (0,0);
    for (i,k) in i_k {
        let limblength = (D[i][n-1] + D[n-1][k] - D[i][k])/2;
        if min_limblength > limblength {
            min_limblength = limblength;
            mini = i;
            mink = k;
        }
    }

    min_limblength
}

fn insertTree((i,j):(usize,usize), distance: usize, tree: &mut BTreeMap<usize, BTreeMap<usize,usize>>) {
    if i==j {
        return
    }
    let mut empty_tree : BTreeMap<usize,usize> = BTreeMap::new();
    tree.entry(i).or_insert(empty_tree.clone());
    tree.entry(j).or_insert(empty_tree.clone());
    tree.get_mut(&i).unwrap().insert(j,distance);
    tree.get_mut(&j).unwrap().insert(i,distance); 
}

fn removeTree((i,j):(usize,usize), tree: &mut BTreeMap<usize, BTreeMap<usize,usize>>) {
    tree.get_mut(&i).unwrap().remove(&j);
    tree.get_mut(&j).unwrap().remove(&i);
}

fn findIK(n:usize, D: &Vec<Vec<usize>>) -> (Option<(usize,usize)>, Option<(usize,usize)>) {
    for i in 0..n-1 {
        for k in i..n-1 {
            if D[i][k] == D[i][n-1] + D[k][n-1] {
                return (Some((i+1,D[i][n-1])),Some((k+1,D[k][n-1])))
            }
        }
    }
    (None, None)
}

fn findAdjacentInternal(n: usize, tree : &mut BTreeMap<usize, BTreeMap<usize,usize>>, distance_map : &Vec<Vec<usize>>) {
    let max_node = tree.keys().max().unwrap().to_owned();
    /* build full tree */
    for i in 1..=n {
        for j in i+1..=n {
            let i_internal = tree[&i].first_key_value().unwrap();
            let j_internal = tree[&j].first_key_value().unwrap();
            let dij = distance_map[i-1][j-1]- i_internal.1 - j_internal.1;
            insertTree((*i_internal.0,*j_internal.0), dij, tree);
        }
    }

    /* build tree only with internal nodes */
    let mut trimmed_tree : BTreeMap<usize,Vec<usize>> = BTreeMap::new();
    for n1 in n+1..=max_node {
        let subtree = trimmed_tree.entry(n1).or_insert(vec![]);
        for n2 in n+1..=max_node {
            if n1 != n2 {subtree.push(n2)}
        }
    }

    /* remove non adjacent internal nodes */
    for n1 in n+1..=max_node {
        for n2 in n1+1..=max_node {
            for n3 in n2+1..=max_node {
                let d12 = tree[&n1][&n2];
                let d13 = tree[&n1][&n3];
                let d23 = tree[&n2][&n3];

                if d12 == d13 + d23 {
                    trimmed_tree.get_mut(&n1).unwrap().retain(|&x| x!=n2);
                    trimmed_tree.get_mut(&n2).unwrap().retain(|&x| x!=n1);
                } else if d13 == d12 + d23 {
                    trimmed_tree.get_mut(&n1).unwrap().retain(|&x| x!=n3);
                    trimmed_tree.get_mut(&n3).unwrap().retain(|&x| x!=n1); 
                } else if d23 == d12 + d13 {
                    trimmed_tree.get_mut(&n2).unwrap().retain(|&x| x!=n3);
                    trimmed_tree.get_mut(&n3).unwrap().retain(|&x| x!=n2);
                }

            }
        }
    }

    /* update tree */
    for internal in n+1..=max_node {
        for other in n+1..=max_node {
            if !trimmed_tree[&internal].contains(&other) {
                removeTree((internal,other), tree);
            }
        }
    }
}

fn attachLeaf(n: usize, limblength: usize, (i, idist): (usize,usize), (k, kdist): (usize,usize), internal:usize, tree: &mut BTreeMap<usize,BTreeMap<usize,usize>>) {
    /* connect current leaf n with internal node */
    insertTree((n,internal), limblength, tree);

    /* connect leaf i */
    if let Some(i_connected) = tree[&i].keys().nth(0) {
        let i_connected = i_connected.to_owned();
        let i_connected_dist = tree[&i][&i_connected];
        if  i_connected <= n { // if leaf i is connected to leaf
            removeTree((i,i_connected), tree);
            insertTree((i,internal), idist, tree);
            insertTree((k,internal), i_connected_dist-idist,tree);
        } else { // if leaf is connected to internal node
            if i_connected_dist > idist { // if leaf(i) is not connected to adjacent internal node
                removeTree((i, i_connected), tree);
                insertTree((i,internal), idist, tree);
            }
            // insertTree((i_connected, internal), idist.abs_diff(i_connected_dist), tree);
        }
    } 
    
    /* connect leaf k */
    if let Some(k_connected) = tree[&k].keys().nth(0) {
        let k_connected = k_connected.to_owned();
        let k_connected_dist = tree[&k][&k_connected];
        if  k_connected <= n { // if leaf k is connected to leaf
            removeTree((k,k_connected), tree);
            insertTree((k,internal), kdist, tree);
            insertTree((i,internal), k_connected_dist-kdist, tree);
        } else { // if leaf is connected to internal node
            if k_connected_dist > kdist { // if leaf(i) is not connected to adjacent internal node
                removeTree((k, k_connected), tree);
                insertTree((k,internal), kdist, tree);
            }
            // insertTree((k_connected, internal), k_connected_dist.abs_diff(kdist), tree);
        }
    } 
}

fn AdditivePhylogeny(n:usize, mut D: Vec<Vec<usize>>, internal:usize) -> BTreeMap<usize,BTreeMap<usize,usize>> {
    if n == 2 {
        /* return the tree consisting of a single edge of length D1,2 */
        let mut tree : BTreeMap<usize, BTreeMap<usize, usize> > = BTreeMap::new();
        insertTree((1,2), D[&n-2][&n-1], &mut tree);
        return tree
    }
    let limblength = computeLimb(n, &D);

    /* Bald */
    for j in 0..n-1 {
        D[j][n-1] -= limblength;
        D[n-1][j] = D[j][n-1];
    }

    /* save i, k */
    let (i_dni,k_dnk) = findIK(n, &D);
    let (i, dni) = i_dni.unwrap();
    let (k, dnk) = k_dnk.unwrap();

    /* Trim */
    let mut trimmedD : Vec<Vec<usize>> = Vec::new();
    for i in 0..n-1 {
        trimmedD.push(vec![]);
        for j in 0..n-1 {
            trimmedD[i].push(D[i][j]);
        }
    }

    let mut tree = AdditivePhylogeny(n-1, trimmedD, internal+1);

    /* attach nodes */
    attachLeaf(n, limblength, (i, dni), (k, dnk), internal, &mut tree);

    return tree
}

pub fn run(content:&Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    let mut D : Vec<Vec<usize>> = Vec::new();
    for i in 1..=n {
        let parsed = _parse(&content[i]);
        D.push(parsed);
    }

    let mut tree = AdditivePhylogeny(n, D.clone() , n+1);
    findAdjacentInternal(n, &mut tree, &D);
    for (n1, map) in tree {
        for (n2, dist) in map {
            println!("{}->{}:{dist}", n1-1, n2-1); 
        }
    }
}