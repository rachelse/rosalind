/* Implement Hierarchical Clustering */
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Tree<T:std::cmp::PartialEq + Clone> {
    pub nodes : Vec<Node<T>>
}

impl<T: std::cmp::PartialEq + Clone > Tree<T> {
    pub fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub fn remove(&mut self, node: &T) -> Node<T> {
        let idx = self.nodes.iter().position(|x| x.data == *node).unwrap();
        let n = self.nodes.remove(idx);
        n
    }

    pub fn delete(&mut self, node: &T) {
        self.nodes.retain(|x| x.data != *node);
    }
}

#[derive(PartialEq, Clone,Debug)]
pub struct Node<T: PartialEq + Clone> {
    parent: Option<NodeID>,
    children : Vec<Option<Node<T>>>,
    pub data : T
}

impl<T:std::cmp::PartialEq + Clone> Node<T> {
    pub fn new(data: &T) -> Self {
        Node {
            parent : None,
            children : vec![None],
            data : data.clone()
        }
    }

    pub fn push_child(&mut self, c: &Node<T>) {
        self.children.push(Some(c.clone()));
        self.children.retain(|x| x != &None);
    }

    pub fn set_parent(&mut self, nodeid: &NodeID) {
        self.parent = Some(nodeid.clone());
    }
}

#[derive(Eq,PartialEq,PartialOrd, Ord,Clone,Debug)]
pub struct NodeID(String);

impl std::fmt::Display for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl NodeID {
    pub fn get(&self) -> &String {
        &self.0
    }
}

fn _parse(line: &String) -> Vec<f32> {
    let parsed : Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
    parsed
}

fn find_closest(D:&BTreeMap<(NodeID,NodeID),f32>) -> (NodeID,NodeID) {
    let mut min_dist = f32::MAX;
    let mut min_idx = (NodeID("None".to_owned()),NodeID("None".to_owned()));
    for ((i,j) ,dist) in D {
        if *dist < min_dist {
            min_dist = *dist;
            min_idx = (i.clone(),j.clone());
        }
    }

    min_idx
}

fn calc_distance(D:&BTreeMap<(NodeID,NodeID),f32>, n1: &NodeID, n2: &NodeID) -> f32 {
    let nodes1 : Vec<&str> = n1.0.split_whitespace().collect();
    let nodes2 : Vec<&str> = n2.0.split_whitespace().collect();
    let mut numerator = 0.0;
    let mut denominator = nodes1.len() * nodes2.len();
    for n1 in &nodes1 {
        for n2 in &nodes2 {
            let n1 = NodeID(n1.to_string());
            let n2 = NodeID(n2.to_string());
            if D.contains_key(&(n1.clone(),n2.clone())) {
                numerator+=D[&(n1.clone(),n2.clone())];
            } else if D.contains_key(&(n2.clone(), n1.clone())) {
                numerator+=D[&(n2.clone(),n1.clone())];
            } else {
                println!("key not found {n1} {n2}");
            }
        }
    }
    numerator/denominator as f32

}

fn HierarchicalCluster(n:usize, D:&mut BTreeMap<(NodeID,NodeID),f32>) {
    let original_distance = D.clone();

    // construct a graph T with n isolated nodes labeled by single elements 1, ... , n
    let mut tree: Tree<NodeID> = Tree {nodes : Vec::new()};

    for i in 1..n+1 {
        let nodeid = NodeID(format!("{i}"));
        let node = Node::new(&nodeid);
        tree.push(node);
    }

    while tree.nodes.len() > 1 {
        //   find the two closest clusters Ci and Cj  
        let min_idx = find_closest(&D);

        //   merge Ci and Cj into a new cluster Cnew with |Ci| + |Cj| elements
        //   add a new node labeled by cluster Cnew to T
        //   connect node Cnew to Ci and Cj by directed edges 
        //   remove Ci and Cj from Clusters 
        let id = format!("{} {}", min_idx.0.0, min_idx.1.0);
        println!("{id}");
        let newid = NodeID(id);
        let mut newnode = Node::new(&newid);
        let mut n1 = tree.remove(&min_idx.0);
        let mut n2 = tree.remove(&min_idx.1);
        n1.set_parent(&newid);
        n2.set_parent(&newid);
        newnode.push_child(&n1);
        newnode.push_child(&n2);

        //   remove the rows and columns of D corresponding to Ci and Cj 
        //   add a row/column to D for Cnew by computing D(Cnew, C) for each C in Clusters 

        for c in &tree.nodes {
            let cid = c.data.clone();
            let new_dist = calc_distance(&original_distance, &newid, &cid);
            D.insert((newid.clone(),cid), new_dist);
        }

        let id1 = n1.data.clone();
        let id2 = n2.data.clone();
        let remove_keys : Vec<(NodeID,NodeID)> = D.iter()
                            .filter(|&(k,v)| k.0==id1 || k.1==id1 || k.0==id2 || k.1==id2)
                            .map(|(k,v)| k.clone()).collect();
        for node in remove_keys {
            D.remove(&node);
        }

        //   add Cnew to Clusters 
        tree.push(newnode);
    }
}

pub fn run(content:&Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    let mut dist_map : BTreeMap<(NodeID,NodeID),f32> = BTreeMap::new();
    for i in 1..content.len() {
        let line = _parse(&content[i]);
        for (j,distance) in line.iter().enumerate() {
            if i-1 < j {
                let ni = NodeID(format!("{}", i));
                let nj = NodeID(format!("{}", j+1));
                dist_map.insert((ni,nj), *distance);
            }
        }
    }
    HierarchicalCluster(n, &mut dist_map);
}