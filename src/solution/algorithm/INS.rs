/*
 * File: INS.rs
 * Project: algorithm
 * File Created: 22nd Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

pub fn run(content: Vec<String>) {
    let n = content.get(0).unwrap().parse::<usize>().unwrap();
    let mut arr: Vec<i32> = content.get(1).unwrap().split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
    let mut cnt = 0;
    let mut k;
    for i in 1..n {
        k=i;
        while k > 0 {
            let nk = arr.get(k).unwrap();
            let nk_1 = arr.get(k-1).unwrap();
            if nk_1 > nk {
                arr.swap(k-1, k);
                k-=1;
                cnt+=1;
            } else {
                break
            }
        }
    }
    println!("{cnt}");
}