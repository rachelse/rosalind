/* Implement SmallParsimony */
use std::collections::BTreeMap;
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
        2 => {return Some('T')},
        3 => {return Some('G')}
        _ => {return None}
    }
}

fn _char_to_index(c: char) -> Option<usize> {
    match c {
        'A' => {return Some(0)},
        'C' => {return Some(1)},
        'T' => {return Some(2)},
        'G' => {return Some(3)},
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

    parent: Option<usize>,
    leftchild: Option<usize>,
    rightchild: Option<usize>
}

impl Node {
    pub fn new(id: String) -> Self {
        let mut node = Node {
            id : 0,
            nodetype : NodeType::None,
            label : "".to_string(),
            parent: None,
            leftchild: None,
            rightchild: None
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
    fn is_root(&self) -> bool {
        if self.parent == None {
            return true
        }
        false
    }

    fn is_leaf(&self) -> bool {
        if self.nodetype == NodeType::Leaf {
            return true
        }
        false
    }
}

fn findRipeNodes(tree: &BTreeMap<usize, Node>, tag: &Vec<usize>) -> Vec<usize> {
    let mut ripe = Vec::new();
    for (v, node) in tree {
        if tag[*v] == 0 {
            if tag[node.rightchild.unwrap()] == 1 && tag[node.leftchild.unwrap()] == 1 {
                ripe.push(*v);
            }
        }
    }
    ripe
}

fn buildTree(lines: &[String]) -> BTreeMap<usize, Node> {
    let mut tree : BTreeMap<usize, Node> = BTreeMap::new();

    /* Build tree */
    let mut used = 0;
    for i in 0..lines.len() {
        let (mut n1, mut n2) = _parse_line(&lines[i]);

        /* assign node number for leaf nodes */
        match n1.nodetype {
            NodeType::Leaf => {n1.id = used;used+=1},
            _ => ()
        }
        match n2.nodetype {
            NodeType::Leaf => {n2.id = used;used+=1},
            _ => ()
        }

        let n1_id = n1.id;
        let n2_id = n2.id;

        tree.entry(n1_id).or_insert(n1);
        tree.entry(n2_id).or_insert(n2);

        /* connect n1 - n2 : find n1 from tree and  */
        if let None = tree[&n1_id].leftchild {
            tree.get_mut(&n1_id).unwrap().leftchild = Some(n2_id);
            tree.get_mut(&n2_id).unwrap().parent = Some(n1_id);
        } else if None == tree[&n1_id].rightchild {
            tree.get_mut(&n1_id).unwrap().rightchild = Some(n2_id); 
            tree.get_mut(&n2_id).unwrap().parent = Some(n1_id);
        } else {
            println!("{n1_id} children are already all occupied");
        }
    }
    tree
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

fn _print(tree:&BTreeMap<usize,Node>, parent_node: &Node) {
    if parent_node.is_leaf() {
        return
    }
    let left_node = &tree[&parent_node.leftchild.unwrap()];
    let left_dist = calc_hammdist(&parent_node.label[..], &left_node.label[..]);
    println!("{}->{}:{}", left_node.label, parent_node.label, left_dist);
    println!("{}->{}:{}", parent_node.label, left_node.label, left_dist);

    let right_node = &tree[&parent_node.rightchild.unwrap()];
    let right_dist = calc_hammdist(&parent_node.label[..], &right_node.label[..]);
    println!("{}->{}:{}", right_node.label, parent_node.label, right_dist);
    println!("{}->{}:{}", parent_node.label, right_node.label, right_dist); 

    _print(tree, left_node);
    _print(tree, right_node);
}

fn printTree(tree: BTreeMap<usize,Node>) {
    let mut root = 0;
    /* find root */
    for (i, node) in &tree {
        if node.is_root() {
            root = *i;
        }
    }
    
    /* print from root */
    let mut parent_node = &tree[&root];
    _print(&tree, parent_node);
}

fn BackTrack(tree: &mut BTreeMap<usize,Node>, parent: usize, parent_idx: usize, sk : &Vec<Vec<usize>>) {
    if tree[&parent].is_leaf() {
        return
    }
    let parent_score = sk[parent][parent_idx];
    let left = tree[&parent].leftchild.unwrap();
    let right = tree[&parent].rightchild.unwrap();

    if tree[&left].is_leaf() || tree[&right].is_leaf() {
        return
    }

    let (mut left_idx, left_score) = sk[left].iter().enumerate().min_by(|&(i1,v1), &(i2,v2)| v1.cmp(v2)).unwrap();
    if *left_score == sk[left][parent_idx] {
        tree.get_mut(&left).unwrap().label.push(_index_to_char(parent_idx).unwrap());
        left_idx = parent_idx;
    } else {
        tree.get_mut(&left).unwrap().label.push(_index_to_char(left_idx).unwrap());
    }
    BackTrack(tree, left, left_idx, sk);

    let (mut right_idx, right_score) = sk[right].iter().enumerate().min_by(|&(i1,v1), &(i2,v2)| v1.cmp(v2)).unwrap();
    if *right_score == sk[right][parent_idx] {
        tree.get_mut(&right).unwrap().label.push(_index_to_char(parent_idx).unwrap());
        right_idx = parent_idx;
    } else {
        tree.get_mut(&right).unwrap().label.push(_index_to_char(right_idx).unwrap());
    }

    BackTrack(tree, right, right_idx, sk);
}

fn SmallParsimony(tree: &mut BTreeMap<usize, Node>, character: &Vec<char>) -> usize {
    /* tag saves Sk(v) : [A,C,T,G] */
    let mut tag = vec![0;tree.len()];
    let mut s_k : Vec<Vec<usize>> = vec![vec![usize::MAX-1;4];tree.len()];
    for (v, node) in tree.into_iter() {
        if let NodeType::Leaf = &node.nodetype {
            tag[*v] = 1;

            for (i,c) in "ACTG".char_indices() {
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
        let daughter = tree[v].leftchild.unwrap();
        let son = tree[v].rightchild.unwrap();
        for k in 0..4 {
            let score_left = calcScore(k, &s_k[daughter]);
            let score_right = calcScore(k, &s_k[son]);
            s_k[*v][k] = score_left + score_right;
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
    //     println!("{i} left{:?} right{:?} {}", node.leftchild,node.rightchild, node.label);
    // }
    printTree(tree);  
}