/* Construct a Trie from a Collection of Patterns */

#[derive(Clone,Debug)]
pub struct Node {
    pub symbol: Option<char>,
    pub id: Option<usize>,
    pub next: Vec<Node>
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: None, symbol: None,
            next :  Vec::new()
        }
    }

    pub fn from(id: usize, symbol: char) -> Node {
        Node {
            id: Some(id),
            symbol: Some(symbol),
            next: Vec::new()
        }
    }

    pub fn find(self, nt: char) -> Option<usize> {
        for (i,child) in self.next.iter().enumerate() {
            if (child.symbol.unwrap() == nt) {
                return Some(i)
            }
        }
        return None;
    }

    pub fn update(&mut self, child: Node) {
        self.next.push(child);
    }

    pub fn print(self) {
        for node in self.next {
            println!("{}->{}:{}", self.id.unwrap(), node.id.unwrap(), node.symbol.unwrap());
            node.print();
        }
    }

    pub fn prefixTrieMatching(&self, text: &str) -> Option<usize> {
        let mut v: &Node = self;

        for (i, symbol) in text.char_indices() {
            if let Some(next_idx) = v.to_owned().find(symbol) {
                v = v.next.get(next_idx).unwrap();
                if v.isLeaf() {
                    return Some(i)
                }
            } else {
                return None
            }
        }
        return None
    }

    pub fn isLeaf(&self) -> bool {
        if (self.next.len() == 0) {
            return true
        } else {
            return false
        }
    }
}


pub fn run(content: Vec<String>) {
    let mut graph: Node = Node::from(0, '\0');
    let mut last = 0;

    for pattern in content {
        let mut curr_node: &mut Node = &mut graph;
        
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

    graph.print();
}