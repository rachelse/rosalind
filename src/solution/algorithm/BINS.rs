/*
 * File: BINS.rs
 * Project: algorighm
 * File Created: 22nd Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

use std::ops::Div;

fn binarySearch(num: i32, array: &Vec<i32>, n: usize) -> i32 {
    let mut idx = n/2;
    let mut prev_idx;
    let mut low = 0;
    let mut upp = n;
    while idx < n{
        let idx_n = array.get(idx).unwrap();
        if idx_n == &num {
            return (idx as i32)+1
        } else if idx_n > &num {
            upp = idx;
        } else {
            low = idx;
        }
        prev_idx = idx;
        idx = (upp+low).div(2);
        if idx == prev_idx {
            break
        }
    }

    return -1
}

pub fn run(content: Vec<String>) {
    let n: usize = content.get(0).unwrap().parse::<usize>().unwrap();
    let m: usize = content.get(1).unwrap().parse::<usize>().unwrap();
    let sorted_array: Vec<i32> = content.get(2).unwrap().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
    let int_list: Vec<i32> = content.get(3).unwrap().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..m {
        let num = *int_list.get(i).unwrap();
        let idx = binarySearch(num, &sorted_array, n);
        print!("{idx} ");
    }
}