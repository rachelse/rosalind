/*
 * File: 2SUM.rs
 * Project: algorithm
 * File Created: 13th Apr 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

use std::collections::HashMap;

use crate::utils::utils::{line2numvec, line2twonum};

use super::MS::mergeSort;

pub fn sum2(arr: &mut Vec<i32>, target: i32, n: usize) -> (Option<usize>, Option<usize>) {
    let mut num2idx = HashMap::new();
    for (idx, num) in arr.iter().enumerate() {
        num2idx.entry(*num).and_modify(|vec:&mut Vec<usize>| vec.push(idx+1)).or_insert(vec![idx+1]);
    }
    let sorted_arr = mergeSort(arr);
    let mut ptr1 =0;
    let mut ptr2 =n-1;
    while ptr1 < ptr2 {
        let num1 = sorted_arr.get(ptr1).unwrap();
        let num2 = sorted_arr.get(ptr2).unwrap();
        let sum2 = num1+num2;
        if target == sum2 {
            let idx1 = num2idx.get_mut(num1).unwrap().pop();
            let idx2 = num2idx.get_mut(num2).unwrap().pop();
            if idx1 < idx2 {
                return (idx1, idx2)
            } else {
                return (idx2, idx1)
            }
        } else if sum2 > target {
            ptr2-=1;
        } else {
            ptr1+=1;
        }
    }
    return (None,None);
}

pub fn run(content:Vec<String>) {
    let (k,n) = line2twonum(&content.get(0).unwrap());
    for i in 1..=k {
        let line = content.get(i).unwrap();
        let mut arr: Vec<i32> = line2numvec(line);
        let (idx1, idx2) = sum2(&mut arr, 0, n);
        if let (Some(i1), Some(i2)) = (idx1, idx2) {
            println!("{i1} {i2}");
        } else {
            println!("-1");
        }
    }
}