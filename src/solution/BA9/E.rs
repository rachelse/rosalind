/* Find the Longest Substring Shared by Two Strings */

use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub struct Tree {
    first : Option<char>,
    rest : Vec<Option<Tree>>
}

impl Tree {
    fn new() -> Self {
        Tree {
            first : None,
            rest : Vec::new()
        }
    }

    fn find(&self, key: char) -> Option<usize> {
        for (i,tree) in self.rest.iter().enumerate() {
            if let Some(t) = tree {
                if t.first.unwrap() == key {
                    return Some(i)
                }
            }
        }
        return None
    }

    fn build_recursive(text:&str) -> Option<Self>{
        if text.len() < 1 {
            return None
        }
        let mut subtree= Tree::new();
        let first = text.chars().nth(0);
        let rest = &text[1..];
        subtree.first = first;
        let child = Tree::build_recursive(rest);
        subtree.rest.push(child);
        return Some(subtree)
    }

    fn construct(&mut self, text: &str) {
        if text.len() == 0 {
            return
        }
        let first = text.chars().nth(0).unwrap();
        if let Some(i) = self.find(first) {
            let mut subtree = self.rest.get_mut(i).unwrap().as_mut().unwrap();
            subtree.construct(&text[1..]);
        } else {
            let subtree = Tree::build_recursive(text);
            self.rest.push(subtree);
        }
    }
    
    fn is_tail(&self) -> bool {
        if self.rest.len() ==0 {
            return true
        } else if self.rest.len() == 1 && self.rest[0] == None {
            return true
        }
        false
    }

    fn is_root(&self) -> bool {
        if self.first == None {
            return true
        }
        false
    }

    fn save_shared_substring(&self, other: &str, prev_substr: String, saver : &mut HashSet<String>) {
        if other.len() == 0 {
            return 
        }

        let first = other.chars().nth(0).unwrap();
        if let Some(idx) = self.find(first) {
            let r = self.rest.get(idx).unwrap().as_ref().unwrap();
            let new_substr = format!("{prev_substr}{first}");
            r.save_shared_substring(&other[1..], new_substr, saver);
        } else {
            saver.insert(prev_substr);
            return
        }
    }
}

pub fn run(content: Vec<String>){
    let text1 = content.get(0).unwrap().to_owned();
    let text2 = content.get(1).unwrap().to_owned();

    /* construct each tree */
    let mut tree1 = Tree::new();
    for i in 0..text1.len() {
        tree1.construct(&text1[i..]);
    }


    /* get uniq substring by comparing tree and other text */
    let mut shared_substr = HashSet::new();
    for i in 0..text2.len() {
        tree1.save_shared_substring(&text2[i..], String::new(), &mut shared_substr);
    }

    let longest = shared_substr.iter().enumerate().max_by(|&(i1,x), &(i2,y)| x.len().cmp(&y.len())).map(|(i,x)| x).unwrap();
    println!("{longest}");
}