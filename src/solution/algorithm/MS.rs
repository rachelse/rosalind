/*
 * File: MS.rs
 * Project: algorithm
 * File Created: 12th Apr 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Merge Sort */
use crate::{print_answer, utils::utils::line2numvec};

use super::MER::mergeArrays;

pub fn mergeSort(arr: &mut [i32]) -> Vec<i32>{
    let arr_size = arr.len();
    if arr_size <= 2 {
        if arr_size == 2 && arr[0] > arr[1] {
            arr.swap(0, 1);
        }
        return arr.to_owned();
    } else {
        let half= arr_size / 2;
        let left = mergeSort(&mut arr[..half]);
        let right = mergeSort(&mut arr[half..]);
        let merged_arr = mergeArrays(&left, &right);
        return merged_arr
    }
}

pub fn run(content: Vec<String>) {
    let n = content.get(0).unwrap().parse::<usize>().unwrap();
    let mut arr: Vec<i32> = line2numvec(content.get(1).unwrap());

    let ms_arr= mergeSort(&mut arr);
    print_answer(ms_arr);
}