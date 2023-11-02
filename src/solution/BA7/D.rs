/* Implement UPGMA */

use std::collections::BTreeMap;

fn _parse(line: &String) -> Vec<f32> {
    let parsed : Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
    parsed
}

#[derive(Debug)]
pub struct Tree {
    pub tree: Vec<Cluster>,
    pub distmap : BTreeMap<(NodeID,NodeID), f32> 
}

impl Tree {
    pub fn new(n: usize, D: &Vec<Vec<f32>>) -> Self {
        let mut tree: Vec<Cluster> = Vec::new();
        let mut distmap : BTreeMap<(NodeID,NodeID), f32> = BTreeMap::new();

        for i in 0..n {
            let nodeid = NodeID(i);
            let node = Cluster::new(&nodeid, true);
            tree.push(node);
        }

        for i in 0..n {
            for j in i+1..n {
                let nodei = NodeID(i);
                let nodej = NodeID(j);
                distmap.insert((nodei.clone(),nodej.clone()), D[i][j]);
                distmap.insert((nodej,nodei), D[j][i]);
            }
        }
        Tree {tree, distmap}
    }

    fn find_closest(&self) -> (NodeID, NodeID, f32) {
        let mut min_dist = &f32::MAX;
        let mut mini : Option<&NodeID> = None;
        let mut minj : Option<&NodeID> = None;
        for ((i,j), dist) in &self.distmap {
            if dist < min_dist {
                min_dist = dist;
                mini = Some(i);
                minj = Some(j);
            }
        }
        (mini.unwrap().clone(), minj.unwrap().clone(), min_dist.to_owned())
    }

    fn merge(&mut self, newid: usize, c1: NodeID, c2: NodeID, age: f32) -> (NodeID, NodeID, Cluster) {
        let idx1= self.tree.iter().position(|x| x.id == c1).unwrap();
        let mut cluster1 = self.tree.remove(idx1);
        let idx2 =self.tree.iter().position(|x| x.id == c2).unwrap();
        let mut cluster2 = self.tree.remove(idx2);

        let new_id = NodeID(newid);
        let mut newcluster = Cluster::new(&new_id, false);
        cluster1.set_parent(&new_id);
        cluster2.set_parent(&new_id);
        newcluster.push_child(&cluster1);
        newcluster.push_child(&cluster2);
        newcluster.set_age(age);

        (cluster1.id, cluster2.id, newcluster)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    pub id : NodeID,
    pub parent: Option<NodeID>,
    pub children: Vec<Option<Cluster>>,
    pub children_list : Vec<usize>,
    pub age: f32

}

impl Cluster {
    pub fn new(data: &NodeID, initial:bool) -> Self {
        let mut list = Vec::new();
        if initial {
            list.push(data.0)
        }
        Cluster {
            id : data.clone(),
            parent : None,
            children : vec![],
            children_list: list,
            age: 0.0
        }
    }

    pub fn push_child(&mut self, c: &Cluster) {
        self.children.push(Some(c.clone()));
        self.children_list.extend(c.children_list.clone());
    }

    pub fn set_parent(&mut self, nodeid: &NodeID) {
        self.parent = Some(nodeid.clone());
    }

    pub fn is_leaf(&self) -> bool {
        if self.children.len() == 0 || self.children.contains(&None) {
            return true
        }
        return false
    }

    pub fn set_age(&mut self, age:f32) {
        self.age = age;
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct NodeID(usize);

impl std::fmt::Display for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl NodeID {
    pub fn get(&self) -> &usize {
        &self.0
    }
}

fn calc_distance(D:&BTreeMap<(NodeID,NodeID),f32>, c1: &Cluster, c2: &Cluster) -> f32 {
    let nodes1 : &Vec<usize> = &c1.children_list;
    let nodes2 : &Vec<usize> = &c2.children_list;
    let mut numerator = 0.0;
    let mut denominator = nodes1.len() * nodes2.len();
    for n1 in nodes1 {
        for n2 in nodes2 {
            let n1 = *n1;
            let n2 = *n2;
            if D.contains_key(&(NodeID(n1),NodeID(n2))) {
                numerator+=D[&(NodeID(n1),NodeID(n2))];
            } else if D.contains_key(&(NodeID(n1),NodeID(n2))) {
                numerator+=D[&(NodeID(n1),NodeID(n2))];
            } else {
                println!("key not found {n1} {n2}");
            }
        }
    }
    numerator/denominator as f32
}

fn print_tree(tree: &Cluster) {
    let mut parent = &tree;
    for child in &parent.children {
        let child_unwrapped = child.as_ref().unwrap();
        let length = parent.age - child_unwrapped.age;
        println!("{:?}->{:?}:{}", child_unwrapped.id.0, parent.id.0, length);
        println!("{:?}->{:?}:{}", parent.id.0, child_unwrapped.id.0, length);
        if !tree.is_leaf() {
            print_tree(child.as_ref().unwrap());
        } 
    }
}

fn UPGMA(D: &Vec<Vec<f32>>, n: usize) -> Cluster {
    // construct a graph T with n isolated nodes labeled by single elements 1, ... , n
    let mut T = Tree::new(n, &D);
    let original_distance = T.distmap.clone();
    let mut id_to_use = n;
    while T.tree.len() > 1 {
        let (ci, cj, dist) = T.find_closest();
        let (c1,c2,newcluster) = T.merge(id_to_use,ci, cj, dist/2.0);

        for c in &T.tree {
            let new_dist = calc_distance(&original_distance, &newcluster, &c);
            T.distmap.insert((newcluster.id.clone(),c.id.clone()), new_dist);
        }

        let remove_keys : Vec<(NodeID,NodeID)> = T.distmap.iter()
                            .filter(|&(k,v)| k.0==c1 || k.1==c1 || k.0==c2 || k.1==c2)
                            .map(|(k,v)| k.clone()).collect();
        for node in remove_keys {
            T.distmap.remove(&node);
        }

        T.tree.push(newcluster);
        id_to_use+=1;
    }

    T.tree.get(0).unwrap().to_owned()
}

pub fn run(content: &Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    let mut D : Vec<Vec<f32>> = Vec::new();
    for i in 1..=n {
        let parsed = _parse(&content[i]);
        D.push(parsed);
    }

    let tree = UPGMA(&D, n);
    print_tree(&tree);
}