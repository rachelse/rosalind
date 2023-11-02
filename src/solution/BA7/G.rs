/* Adapt SmallParsimony to Unrooted Trees */
use std::collections::{BTreeMap, HashMap};
use crate::biology::kmer::calc_hammdist;

fn _parse_line(line: &String) -> (Node, Node) {
    let parsed: Vec<&str> = line.split("->").collect();
    let mut n1 = Node::new(parsed[0].to_string());
    let mut n2 = Node::new(parsed[1].to_string());
    (n1,n2)
}

fn _index_to_char(i:usize) -> Option<char> {
    match i {
        0 => {return Some('A')},
        1 => {return Some('C')},
        2 => {return Some('G')},
        3 => {return Some('T')}
        _ => {return None}
    }
}

fn _char_to_index(c: char) -> Option<usize> {
    match c {
        'A' => {return Some(0)},
        'C' => {return Some(1)},
        'G' => {return Some(2)},
        'T' => {return Some(3)},
        _ => {return None}
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NodeType {
    Internal,
    Leaf,
    None
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    pub id : usize,
    pub nodetype : NodeType,
    label : String,
    neighbors: Vec<usize>,
    // internal: Vec<usize>,
    // children: Vec<usize>
}

impl Node {
    pub fn new(id: String) -> Self {
        let mut node = Node {
            id : 0,
            nodetype : NodeType::None,
            label : "".to_string(),
            neighbors: Vec::new()
            // internal: Vec::new(),
            // children: Vec::new()
        };

        if let Ok(num) = id.parse::<usize>() {
            node.nodetype = NodeType::Internal;
            node.id = num;
        } else {
            node.nodetype = NodeType::Leaf;
            node.label = id.to_string();
        }

        node
    }

    fn is_leaf(&self) -> bool {
        if self.nodetype == NodeType::Leaf {
            return true
        }
        false
    }
}

fn buildTree(lines: &[String]) -> BTreeMap<usize, Node> {
    let mut tree : BTreeMap<usize, Node> = BTreeMap::new();

    /* Build tree */
    let mut used = 0;
    let mut patterns : HashMap<String, usize> = HashMap::new();
    for i in 0..lines.len() {
        let (mut n1, mut n2) = _parse_line(&lines[i]);

        /* assign node number for leaf nodes */
        if let NodeType::Leaf = &n1.nodetype {
            if !patterns.contains_key(&n1.label) {
                n1.id = used;
                patterns.insert(n1.label.clone(), used);
                used +=1;
            } else {
                n1 = tree[&patterns[&n1.label]].clone();
            }
        }

        if let NodeType::Leaf = &n2.nodetype {
            if !patterns.contains_key(&n2.label) {
                n2.id = used;
                patterns.insert(n2.label.clone(), used);
                used +=1;
            } else {
                n2 = tree[&patterns[&n2.label]].clone();
            }
        }

        let n1_id = n1.id;
        let n2_id = n2.id;

        tree.entry(n1_id).or_insert(n1.clone());
        tree.entry(n2_id).or_insert(n2.clone());

        /* connect n1 - n2 : find n1 from tree and  */
        tree.get_mut(&n1_id).unwrap().neighbors.push(n2_id);
    }
    tree
}

fn findRipeNodes(tree: &BTreeMap<usize, Node>, tag: &Vec<usize>) -> Vec<usize> {
    let mut ripe = Vec::new();
    for (v, node) in tree {
        if tag[*v] == 0 {
            let mut tagged = 0;
            for n in &node.neighbors {
                if tag[*n] == 1 {tagged+=1;}
            }
            if tagged >= 2 { ripe.push(*v);}
        }
    }
    ripe
}

fn calcScore(k: usize, child: &Vec<usize>) -> usize {
    let mut score = usize::MAX;
    for i in 0..child.len() {
        let mut tmp = child[i];
        if i != k {tmp+=1;}
        if tmp < score {
            score = tmp;
        }
    }
    score
}

fn printTree(tree: BTreeMap<usize,Node>) {
    for (i, node) in &tree {
        for neighbor in &node.neighbors {
            let neighbor_node = &tree[&neighbor];
            let dist = calc_hammdist(&node.label, &neighbor_node.label);
            println!("{}->{}:{}", node.label, neighbor_node.label, dist);
        }
    }
}

fn BackTrack(tree: &mut BTreeMap<usize,Node>, parent: usize, parent_idx: usize, sk : &Vec<Vec<usize>>) {
    if tree[&parent].is_leaf() {
        return
    }

    let pattern_len = tree[&parent].label.len();

    for neighbor in tree[&parent].neighbors.clone() {
        if tree[&neighbor].label.len() < pattern_len {
            let (mut neighbor_idx, neighbor_score) = sk[neighbor].iter().enumerate().min_by(|&(i1,v1), &(i2,v2)| v1.cmp(v2)).unwrap();
            if *neighbor_score == sk[neighbor][parent_idx] {
                tree.get_mut(&neighbor).unwrap().label.push(_index_to_char(parent_idx).unwrap());
                neighbor_idx = parent_idx;
            } else {
                tree.get_mut(&neighbor).unwrap().label.push(_index_to_char(neighbor_idx).unwrap());
            }
            BackTrack(tree, neighbor, neighbor_idx, sk);
        }
    }
}

fn SmallParsimony(tree: &mut BTreeMap<usize, Node>, character: &Vec<char>) -> usize {
    /* tag saves Sk(v) : [A,C,G,T] */
    let mut tag = vec![0;tree.len()];
    let mut s_k : Vec<Vec<usize>> = vec![vec![usize::MAX-1;4];tree.len()];
    for (v, node) in tree.into_iter() {
        if let NodeType::Leaf = &node.nodetype {
            tag[*v] = 1;

            for (i,c) in "ACGT".char_indices() {
                if c == character[*v] {
                    s_k[*v][i] = 0;
                }
            }
        }
    }

    /* calculate scores over internal nodes */
    let mut ripe_nodes = findRipeNodes(&tree, &tag);
    let mut root = 0;
    while ripe_nodes.len() > 0 {
        let v = ripe_nodes.first().unwrap();
        root = *v;
        tag[*v] = 1;

        let mut tagged_neighbors = Vec::new();
        for n in &tree[v].neighbors {
            if tag[*n] == 1 {
                tagged_neighbors.push(*n);
            }
        }

        for k in 0..4 {
            let mut tmp_score = 0;
            for n in &tagged_neighbors {
                tmp_score += calcScore(k, &s_k[*n]);
            }
            s_k[*v][k] = tmp_score;
        }

        ripe_nodes = findRipeNodes(tree, &tag);
    }

    /* Backtrack from root to internal nodes and assign labels */
    let (root_idx , root_score) = s_k[root].iter().enumerate().min_by(|(i1,v1), (i2,v2)| v1.cmp(v2)).unwrap();
    let root_char =_index_to_char(root_idx).unwrap();
    tree.get_mut(&root).unwrap().label.push(root_char);
    BackTrack(tree, root, root_idx, &s_k);
    
    /* minimum parsimony score */
    *root_score
}

pub fn run(content: &Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();

    let mut tree = buildTree(&content[1..]);

    let mut label_length = 0;
    if let NodeType::Leaf = &tree[&0].nodetype {
        label_length = tree[&0].label.len();
    }
    
    /* build vector with labels */
    let mut labels : Vec<Vec<char>> = Vec::new();
    for i in 0..label_length {
        labels.push(Vec::new());
    }

    for i in 0..n {
        match &tree[&i].nodetype {
            NodeType::Leaf => {
                for (i,c) in tree[&i].label.char_indices() {
                    labels[i].push(c);
            }},
            _ => ()
        }
    }

    let mut minimum_score = 0;
    for i in 0..label_length{
        let score = SmallParsimony(&mut tree, &labels[i]);
        minimum_score += score;
    }

    println!("{minimum_score}");
    // for (i, node) in &tree {
    //     println!("{i} neighbors{:?} {}", node.neighbors, node.label);
    // }
    printTree(tree);  
}