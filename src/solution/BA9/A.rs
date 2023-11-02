/* Construct a Trie from a Collection of Patterns */

/*
TRIECONSTRUCTION(Patterns)
        Trie ← a graph consisting of a single node root
        for each string Pattern in Patterns
            currentNode ← root
            for i ← 1 to |Pattern|
                if there is an outgoing edge from currentNode with label currentSymbol
                    currentNode ← ending node of this edge
                else
                    add a new node newNode to Trie
                    add a new edge from currentNode to newNode with label currentSymbol
                    currentNode ← newNode
        return Trie
*/

fn TRIEConstruction(patterns: &Vec<String>) {
    // let trie = Vec::new();
    for pattern in patterns {

    }
} 
pub fn run(content: &Vec<String>) {

}