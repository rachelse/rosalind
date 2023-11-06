/* Pattern Matching with the Suffix Array */

use std::collections::BTreeMap;
use crate::print_answer;

pub fn run(content: Vec<String>) {
    let text = content[0].to_owned();

    /* Construct suffix array */
    let mut suffix_map = BTreeMap::new();
    for i in 0..text.len() {
        suffix_map.insert( &text[i..], i);
    }

    /* match patterns */
    let mut matched = Vec::new();
    for pattern in &content[1..] {
        for (suffix, i) in &suffix_map {
            if suffix.len() >= pattern.len() && &suffix[..pattern.len()] == pattern.as_str() {
                matched.push(i);
            }
        }
    }
    matched.sort();
    print_answer(matched);
}