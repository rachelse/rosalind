/*
 * File: INI3.rs
 * Project: INI
 * File Created: 12th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

pub fn run(content: Vec<String>) {
    let s = content.get(0).unwrap();
    let indices : Vec<usize> = content.get(1).unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let (a,b,c,d) = (
        *indices.get(0).unwrap(), *indices.get(1).unwrap(), 
        *indices.get(2).unwrap(), *indices.get(3).unwrap());
    
    println!("{} {}", &s[a..=b], &s[c..=d]);
}