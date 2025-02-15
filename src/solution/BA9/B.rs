/*
 * File: B.rs
 * Project: BA9
 * File Created: 15th Feb 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */
/* Implement TrieMatching */
use super::A::Node;

pub fn run(content: Vec<String>) {
    let mut trie: Node = Node::from(0, '\0');
    let mut last = 0;
    
    for i in 1..content.len() {
        let pattern = content.get(i).unwrap();
        let mut curr_node: &mut Node = &mut trie;
        
        for nt in pattern.chars() {
            let mut new_idx = curr_node.to_owned().find(nt);
            if let None = new_idx {
                last += 1;
                let new_node = Node::from(last, nt);
                curr_node.update(new_node);
                new_idx = Some(curr_node.next.len()-1);
            }
            curr_node = curr_node.next.get_mut(new_idx.unwrap()).unwrap();
        }
    }

    // TrieMatching(Text, Trie)
    let mut text = &content.get(0).unwrap()[..];
    let mut prefix_idx = 0;
    while text.len() > 0 {
        let nth= trie.prefixTrieMatching(text);
        if let Some(d) = nth {
            print!("{} ", prefix_idx);
        }
        text = &text[1..];
        prefix_idx+=1;
    }

}