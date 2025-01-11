/*
 * File: INI6.rs
 * Project: INI
 * File Created: 12th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */
use std::collections::HashMap;

pub fn run(content: Vec<String>) {
    let mut map : HashMap<String, usize> = HashMap::new();

    let words= content.get(0).unwrap().split_whitespace();
    for word in words.into_iter() {
        map.entry(word.to_string()).and_modify(|x| *x+=1).or_insert(1);
    }

    for (k,v) in map.iter() {
        println!("{k} {v}");
    }
}