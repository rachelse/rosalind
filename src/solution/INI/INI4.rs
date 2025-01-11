/*
 * File: INI4.rs
 * Project: INI
 * File Created: 12th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

pub fn run(content: Vec<String>) {
    let num : Vec<usize> = content.iter().nth(0).unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let (mut a,b) = (*num.get(0).unwrap(), *num.last().unwrap());
    let mut sum = 0;

    if (a%2 == 0) {
        a+=1;
    }
    for i in (a..=b).step_by(2) {
        sum+=i;
    }
    
    println!("{sum}");
}