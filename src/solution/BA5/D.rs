/* Find the Longest Path in a DAG */

use std::collections::HashMap;
use std::cmp;
use std::hash::Hash;

#[derive(Debug)]
struct DAG {
    source: usize,
    sink: usize,
    map: HashMap<usize,HashMap<usize,usize>>,

    distance : Vec<usize>,
    predecessor: Vec<usize>,
}

impl DAG {
    fn _parse_line(line:&String) -> (usize,usize,usize) {
        let n1_n2w: Vec<&str> = line.split("->").collect();
        let n1 = n1_n2w[0].parse::<usize>().unwrap();
        let n2_w: Vec<usize> = n1_n2w[1].split(":").map(|x| x.parse::<usize>().unwrap()).collect();
        let n2 = n2_w[0];
        let w = n2_w[1];
        (n1,n2,w)
    }

    pub fn build(content:&Vec<String>) -> Self {
        let mut max_node = 0;

        let source = content[0].parse::<usize>().unwrap();
        let sink = content[1].parse::<usize>().unwrap();
        let mut map: HashMap<usize, HashMap<usize,usize>> = HashMap::new();

        for i in 2..content.len() {
            let mut sub: HashMap<usize,usize> = HashMap::new();
            let (n1,n2,w) = DAG::_parse_line(&content[i]);
            sub.insert(n2, w);
            map.entry(n1).or_insert(sub).insert(n2, w) ;
            if cmp::max(n1,n2) > max_node {
                max_node = cmp::max(n1,n2);
            }
        }

        let distance: Vec<usize> = vec![0;max_node+1];
        let predecessor: Vec<usize> = vec![sink;max_node+1];
        let dag = DAG {source, sink, map, distance, predecessor};
        dag
    }

    pub fn find_neighbor(&self, node: usize) -> Vec<usize> {
        let mut neighbor = Vec::new();
        if self.map.contains_key(&node) {
            for k in self.map[&node].keys() {
                neighbor.push(*k);
            }
        }
        neighbor
    }

    pub fn find_path(&mut self, n1: usize) {
        if n1 == self.sink {
            return
        } 
        let neighbors = self.find_neighbor(n1);
        if neighbors.len() == 0 {
            return
        }

        for n2 in neighbors {
            let dist = self.map[&n1][&n2];
            self.update(n1,n2,dist);
            self.find_path(n2);
        }
    }

    fn update(&mut self, n1: usize, n2: usize, dist:usize){
        if self.distance[n2] < self.distance[n1] + dist {
            self.distance[n2] = self.distance[n1] + dist;
            self.predecessor[n2] = n1;
        }
    }

    fn longest_path(&self) {
        let mut n1 = self.sink;
        let mut path = format!("{n1}");
        while n1 != self.source {
            n1 = self.predecessor[n1];
            path = format!("{n1}->{path}");
        }
        println!("{}", self.distance[self.sink]);
        println!("{path}");
    }

}


pub fn run(content:&Vec<String>) {
    let mut dag= DAG::build(&content);
    dag.find_path(dag.source);
    dag.longest_path();
}