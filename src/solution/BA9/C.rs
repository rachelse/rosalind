/* Construct the Suffix Tree of a String */
// use std::{cell::RefCell, rc::Rc};

// #[derive(Debug, Clone)]
// pub enum NodeID<> {
//     Root,
//     ID(char)
// }

// #[derive(Debug, Clone)]
// pub struct Node {
//     val : NodeID,
//     children : Vec<Box<Node>>
// }

// impl Node {
//     pub fn new(val: NodeID) -> Self {
//         Node {
//             val,
//             children: vec![]
//         }
//     }

//     pub fn insert(&self, new_val : &str) {

//     }
// }

// fn SuffixTree(mut T: Node, text: String) {
//     let mut prev_node = &mut Box::new(T);
//     for c in text.chars() {
//         let mut new_node = Node::new(NodeID::ID(c));
//         let mut node_ptr = Box::new(new_node);
//         prev_node.children.push(node_ptr);

//         if let Some(child) = prev_node.children.last_mut() {
//             prev_node = child;
//         }
//     }
//     // for i in 0..text.len() {
//     //     for c in text[i..].chars() {
//     //         if T.
//     //     }

//     // }
// }

// pub fn run(content: &Vec<String>){
//     let text = content[0].to_owned();
//     let mut tree = Node::new(NodeID::Root);
//     SuffixTree(tree, text);
// }