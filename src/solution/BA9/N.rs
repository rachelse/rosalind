/* Find All Occurrences of a Collection of Patterns in a String */

use crate::print_answer;
use crate::solution::BA9::{I,M,Q};

pub fn idx_to_symbol(idx: usize) -> Option<char> {
    match idx {
        0 => {return Some('&')},
        1 => {return Some('A')},
        2 => {return Some('C')},
        3 => {return Some('G')},
        4 => {return Some('T')},
        _ => {return None}
    }
}

pub fn run(content: Vec<String>) {
    let text = &content[0];

    let bwt_text = I::BWT(text);
    let mut suffix_array = Q::SuffixArray(text);

    let last_col: Vec<char> = bwt_text.chars().collect();
    let mut first_col = last_col.clone();
    first_col.sort();

    let first_occurence = M::firstOccurence(&first_col);
    // let cnt_matrix = M::countSymbol(&last_col);

    let mut starting_points: Vec<usize> = Vec::new();

    for pattern in &content[1..] {
        let first = &pattern.chars().nth(0).unwrap();
        let mut i = first_occurence[first];
        let mut end = 0;
        if let Some(n) = idx_to_symbol(M::symbol_to_idx(*first).unwrap()+1) {
            end = first_occurence[&n];
        } else {
            end = bwt_text.len();
        }
        for suffix_idx in i..end {
            let suffix = suffix_array[suffix_idx];
            if suffix+pattern.len() > suffix_array.len() {
                continue
            }
            if &text[suffix..suffix+pattern.len()] == pattern {
                starting_points.push(suffix);
            }
        }
    }
    starting_points.sort();
    print_answer(starting_points);
}