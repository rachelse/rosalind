/*
 * File: INI2.rs
 * Project: INI
 * File Created: 12th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

pub fn run(content: Vec<String>) {
    let num : Vec<usize> = content.iter().nth(0).unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let ans = num.get(0).unwrap().pow(2) + num.get(1).unwrap().pow(2);
    println!("{ans}");
}