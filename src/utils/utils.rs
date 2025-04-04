use core::str;
/*
 * File: utils.rs
 * Project: utils
 * File Created: 4th Apr 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

pub fn line2numvec<T: std::str::FromStr>(line: &String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line.split_ascii_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}


pub fn line2twonum(line: &String) -> (usize, usize) {
    let numbers: Vec<usize> = line.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let (n1, n2) = (*numbers.get(0).unwrap(), *numbers.get(1).unwrap());
    return (n1,n2)
}

