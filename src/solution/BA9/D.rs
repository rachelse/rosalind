/* Find the Longest Repeat in a String */

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Tree {
    first : Option<String>,
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
                if t.first.as_ref().unwrap().chars().nth(0).unwrap() == key {
                    return Some(i)
                }
            }
        }
        return None
    }

    fn get_rest_text(&self, i:usize) -> String {
        let text = self.rest.get(i).unwrap().as_ref().unwrap();
        text.first.as_ref().unwrap().clone()
    }

    fn get_rest(&mut self, i: usize) -> Tree {
        let child = self.rest.remove(i).unwrap();
        child
    }

    fn construct(&mut self, text: &str) {
        if text.len() == 0 {
            self.rest.push(None);
            return
        }

        let first = text.chars().nth(0).unwrap();
        if let Some(i) = self.find(first) { /* if match in rest */
            /* how long the same substring is */
            let prev_text = self.get_rest_text(i);
            let mut idx = 1;
            let mut len_overlap = 0;
            
            while idx <= text.len() && idx <= prev_text.len() && text[..idx] == prev_text[..idx]{
                len_overlap = idx;
                idx+=1;
            }

            let mut prev_child = self.get_rest(i);
            let mut child = Tree::new();
            child.first = Some(prev_text[..len_overlap].to_string());

            /* update previous child node */
            if len_overlap == prev_text.len() {
                prev_child.construct(&text[len_overlap..]);
                for pc in prev_child.rest {
                    child.rest.push(pc);
                }
            } else {
                prev_child.first = Some(prev_text[len_overlap..].to_string());
                child.construct(&text[len_overlap..]);
                child.rest.push(Some(prev_child));
            }
            
            self.rest.push(Some(child));

        } else { /* if matching node not found */
            let mut subtree = Tree::new();
            subtree.first = Some(text.to_string());
            self.rest.push(Some(subtree));
        }
    }
    
    fn is_tail(&self) -> bool {
        if self.rest.len() < 1 {
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

    fn find_repeat(&self, repeat:String, saver : &mut Vec<String>) {
        if self.is_tail() {
            saver.push(repeat);
            return
        }

        let mut substr = repeat.to_string();
        if !self.is_root() && !self.is_tail() {
            let tmp = &self.first.as_ref().unwrap()[..];
            substr.push_str(tmp);
        }

        for r in &self.rest {
            if let Some(tree) = r {
                tree.find_repeat(substr.clone(), saver);
            }
        }
    }

}

pub fn run(content: Vec<String>){
    let text = content.get(0).unwrap().to_owned();
    let mut tree = Tree::new();

    for i in 0..text.len() {
        tree.construct(&text[i..]);
    }
    let mut repeat_saver = Vec::new();
    tree.find_repeat("".to_string(), &mut repeat_saver);

    let mut longest_idx = 0;
    let mut longest_len = 0;
    for (i ,repeat) in repeat_saver.iter().enumerate() {
        if repeat.len() > longest_len {
            longest_len = repeat.len();
            longest_idx = i;
        }
    }

    println!("{}", repeat_saver[longest_idx]);
}