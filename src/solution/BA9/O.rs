/*
 * File: O.rs
 * Project: BA9
 * File Created: 20th Feb 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */
/* Find All Approximate Occurrences of a Collection of Patterns in a String */
use std::collections::HashSet;
use crate::print_answer;
use crate::solution::BA9::{I,M,Q,N};

fn calcDistance(subtext: &str, pattern: &str) -> usize {
    let mut dist = 0;
    for (a, b) in subtext.chars().zip(pattern.chars()) {
        if (a!=b) {
            dist+=1;
        }
    }
    return dist
}

pub fn run(content: Vec<String>) {
    let text = &content[0];
    let patterns: Vec<&str> = content[1].split_whitespace().collect();
    let d = content[2].parse::<usize>().unwrap();

    let bwt_text = I::BWT(text);
    let mut suffix_array = Q::SuffixArray(text);
    let last_col: Vec<char> = bwt_text.chars().collect();
    let mut first_col = last_col.clone();
    first_col.sort();

    let first_occurence = M::firstOccurence(&first_col);
    let mut starting_points: Vec<usize> = Vec::new();

    for pattern in patterns {
        let k = pattern.len().div_ceil(d+1);
        let mut idx = 0;
        let patlen = pattern.len();
        let mut local_startingpoints: HashSet<usize> = HashSet::new();

        // For each subpatterns in pattern
        while idx < patlen {
            
            let subpattern = &pattern[idx..(idx+k).min(patlen)];
            let first = subpattern.chars().nth(0).unwrap();
            let mut start = *first_occurence.get(&first).unwrap();
            let mut end = 0;

            if let Some(n) = N::idx_to_symbol(M::symbol_to_idx(first).unwrap()+1) {
                end = *first_occurence.get(&n).unwrap();
            } else {
                end = bwt_text.len();
            }

            for i in start..end {
                let sub_suffix = *suffix_array.get(i).unwrap();

                // Check if subpattern is in boundary of text
                if (sub_suffix < idx) || (sub_suffix-idx+pattern.len() > suffix_array.len()) {
                    continue
                }
                
                let suffix = sub_suffix - idx;
                if &text[sub_suffix..sub_suffix+subpattern.len()] == subpattern {
                    let dist = calcDistance(&text[suffix..suffix+patlen], pattern);
                    if (dist <= d) {
                        local_startingpoints.insert(suffix);
                    }
                }
            }
            idx+=subpattern.len();
        }

        for i in local_startingpoints {
            starting_points.push(i);
        }
    }
    starting_points.sort();
    print_answer(starting_points);
}