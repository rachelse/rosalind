/*
 * File: R.rs
 * Project: BA9
 * File Created: 3rd Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Construct a Suffix Tree from a Suffix Array */

#[derive(Debug, Clone)]
struct Node {
    id: Option<usize>,
    text: String,
    descent: usize,
    children: Vec<Node>
}

impl Node {
    fn new(text: &str, suffix: Option<usize>, descent: usize) -> Self {
        Node {
            id: suffix,
            text: text.to_string(),
            descent: descent, children: Vec::new()
        }
    }
    fn add_child(&mut self, lcp: usize, child: Node) {
        let (mut rightmost, mut d) = self.find_rightmost(lcp);
        rightmost.children.push(child);
    }

    fn split(&mut self, lcp: usize, text: &str, suffix: Option<usize>) {
        let (mut rightmost, mut d) = self.find_rightmost(lcp);
        let descent = d - rightmost.text.len();
        if (d > lcp) {
            let mut remain = rightmost.clone();
            remain.text = remain.text[(lcp-descent)..].to_string();
            rightmost.text = rightmost.text[..(lcp-descent)].to_string();
            rightmost.id = None;
            rightmost.descent = lcp;
            rightmost.children = vec![remain];
        }
        let child = Node::new(&text[lcp..], suffix, text.len());
        rightmost.children.push(child);
    }
    fn find_rightmost(&mut self, lcp: usize) -> (&mut Self, usize) {
        let mut rightmost = self;
        let mut d = rightmost.descent;
        while d < lcp {
            rightmost = rightmost.children.iter_mut().last().unwrap();
            d = rightmost.descent;
        }
        return (rightmost, d)
    }
    fn traverse(&self) {
        if self.text.len() > 0 {
            println!("{}", self.text);
        }
        for child in &self.children {
            child.traverse();
        }

    }
}

pub fn run(content: Vec<String>) {
    let text = &content[0];
    let suffix_array: Vec<usize> = content[1].split(", ").map(|x| x.parse::<usize>().unwrap()).collect();
    let lcp_array: Vec<usize> = content[2].split(", ").map(|x| x.parse::<usize>().unwrap()).collect();
    let text_len = text.len();

    let mut suffix_tree: Node = Node::new("", None, 0);

    for i in 0..text_len {
        let lcp = *lcp_array.get(i).unwrap();
        let suffix = *suffix_array.get(i).unwrap();
        let mut substr: &str = &text[suffix..];
        if (i == 0) {
            substr = &text[suffix+lcp..];
            let node: Node = Node::new(substr, Some(suffix), substr.len());
            suffix_tree.add_child(lcp, node);
            continue;
        }

        let lcp_1 = *lcp_array.get(i-1).unwrap();
        if lcp == lcp_1 {
            let child: Node = Node::new(&substr[lcp..], Some(suffix), substr.len()); 
            suffix_tree.add_child(lcp, child);
        } else if (lcp_1 > lcp) {
            suffix_tree.split(lcp, substr, Some(suffix));
        } else {
            suffix_tree.split(lcp, substr, Some(suffix));
        }
    }
    suffix_tree.traverse();
}
