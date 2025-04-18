/*
 * File: MER.rs
 * Project: algorithm
 * File Created: 4th Apr 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */


/* Merge Two Sorted Arrays */
use crate::{print_answer, utils::utils::line2numvec};

pub fn mergeArrays<
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug + std::marker::Copy
    >(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let mut merged_arr: Vec<T> = Vec::new();

    let mut iter1 = arr1.into_iter();
    let mut iter2 = arr2.into_iter();
    let mut v1 = iter1.next();
    let mut v2= iter2.next();

    while !(v1 == None &&  v2 == None) {
        if v1 == None {
            merged_arr.push(*v2.unwrap());
            v2 = iter2.next();
        } else if v2 == None {
            merged_arr.push(*v1.unwrap());
            v1 = iter1.next();
        } else {
            if v1.unwrap() < v2.unwrap() {
                merged_arr.push(*v1.unwrap());
                v1 = iter1.next();
            } else {
                merged_arr.push(*v2.unwrap());
                v2 = iter2.next();
            }
        }
    }
    return merged_arr;
}

pub fn run(content: Vec<String>) {
    let n1 = content.get(0).unwrap().parse::<usize>().unwrap();
    let arr1: Vec<i32> = line2numvec(content.get(1).unwrap());
    let n2 = content.get(2).unwrap().parse::<usize>().unwrap(); 
    let arr2: Vec<i32> = line2numvec(content.get(3).unwrap());
    let merged_arr = mergeArrays(&arr1[..], &arr2[..]);

    print_answer(merged_arr);    
}
