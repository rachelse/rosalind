/*
 * File: P.rs
 * Project: BA9
 * File Created: 2nd Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Implement TreeColoring */

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeColor {
    Gray,
    Red,
    Blue,
    Purple   
}

impl NodeColor {
    fn from(clr:&str) -> NodeColor{
        match clr {
            "red" => {return NodeColor::Red},
            "blue" => {return NodeColor::Blue},
            "purple" => {return NodeColor::Purple},
            _ => {return NodeColor::Gray}
        }
    }
    fn to(&self) -> &'static str {
        match self {
            &NodeColor::Blue => {return "blue"},
            &NodeColor::Red => {return "red"},
            &NodeColor::Purple => {return "purple"},
            &NodeColor::Gray => {return "gray"}
        }
    }
}

#[derive(Debug)]
struct Node {
    id: usize,
    color: NodeColor,
    ripe: bool,
    children: Vec<usize>
}

impl Node {
    fn new(id: usize, raw_children: &str) -> Node {
        let mut children: Vec<usize> = Vec::new();
        match raw_children {
            "{}" => {},
            _ => {children = raw_children.split(",").map(|x|x.parse::<usize>().unwrap()).collect();}
        }
        Node {
            id, 
            children,
            ripe: true,
            color: NodeColor::Gray
        }
    }

    fn setClr(&mut self, clr: NodeColor) {
        self.color = clr;
        self.ripe = false;
    }

    fn isRipe(&self) -> bool {
        self.ripe
    }
}

fn parse_input(input: &Vec<String>, tree: &mut Vec<Node>) {
    const DIVIDER: &str = "-";
    let mut idx = 0;
    let mut adj_list : bool = true;

    while adj_list {
        let line = input.get(idx).unwrap();
        if let DIVIDER = &line[..] {
            adj_list = false;
        } else {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let id = parts.get(0).unwrap().parse::<usize>().unwrap();
            let node = Node::new(id, parts.get(1).unwrap());

            match tree.len() {
                id => {tree.push(node);},
                _ => {panic!("Error occured while parsing input")},
            }
        }
        idx+=1;
    }

    while idx < input.len() {
        let line = input.get(idx).unwrap();
        let parts: Vec<&str> = line.split(": ").collect();
        let id = parts.get(0).unwrap().parse::<usize>().unwrap();
        let _clr = *parts.get(1).unwrap();
        let mut clr: NodeColor = NodeColor::from(_clr);
        tree.get_mut(id).unwrap().setClr(clr);
        idx+=1;
    }
}

fn get_ripes(tree: &Vec<Node>) -> Vec<usize> {
    let mut ripe_nodes = Vec::new();
    for i in 0..tree.len() {
        // Set Ripe flat
        let node = tree.get(i).unwrap();
        let mut is_ripe: bool = node.isRipe();
        if !is_ripe {
            continue
        } else {
            for child in &node.children {
                if (tree.get(*child).unwrap().isRipe()) {
                    is_ripe = false;
                    break;
                }
            }
            if is_ripe {
                ripe_nodes.push(node.id);
            }
        }
    }
    return ripe_nodes
}

pub fn run(content: Vec<String>) {
    let mut tree: Vec<Node> = Vec::new();
    parse_input(&content, &mut tree);
    let mut ripe_nodes = get_ripes(&tree);

    while ripe_nodes.len() > 0 {
        for idx in &ripe_nodes {
            let children = tree.get(*idx).unwrap().children.clone();
            let mut clr = tree.get(*children.first().unwrap()).unwrap().color;
            for i in 1..children.len() {
                let _clr =tree.get(*children.iter().nth(i).unwrap()).unwrap().color; 
                if clr != _clr {
                    clr = NodeColor::Purple;
                    break;
                }
            }
            let node = tree.get_mut(*idx).unwrap();
            node.setClr(clr);
        }
        ripe_nodes = get_ripes(&tree);
    }

    for (i, node) in tree.iter().enumerate() {
        println!("{i}: {}", node.color.to());
    }
}