/*
 * File: MAJ.rs
 * Project: algorithm
 * File Created: 4th Apr 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */
/* Majority Element */
use crate::utils::utils;
use std::collections::HashMap;
pub fn run(content: Vec<String>) {
    let (k,n) = utils::line2twonum(content.get(0).unwrap());
    let mut num_counter: HashMap<usize, usize> = HashMap::new();
    for i in 1..=k {
        let line = content.get(i).unwrap();
        let numbers = utils::line2numvec(line);

        for number in numbers {
            num_counter.entry(number).and_modify(|x| *x+=1).or_insert(1);
        }
        let (mut max_key, mut max_cnt) = (0,0);
        for (k,v) in &num_counter {
            if *v > max_cnt {
                max_key = *k;
                max_cnt = *v;
            }
        }
        if (max_cnt > n/2) {
            print!("{max_key} ");
        } else {
            print!("-1 ");
        }

        num_counter.clear();
    }
}
