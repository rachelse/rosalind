/* Construct the Suffix Array of a String */
use std::collections::BTreeMap;

pub fn run(content: Vec<String>) {
    let text = content[0].to_owned();
    let mut suffix_map = BTreeMap::new();
    for i in 0..text.len() {
        suffix_map.insert( &text[i..], i);
    }

    for (suffix, idx) in suffix_map {
        print!("{idx}, ");
    }
    println!();
}