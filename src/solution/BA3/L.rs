/*
 * File: L.rs
 * Project: BA3
 * File Created: 6th Feb 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Construct a String Spelled by a Gapped Genome Path */

use super::J::VirtualKmer;

fn stringSpelledByPatterns(kmers: &Vec<String>, k: usize) -> String {
    let mut string = String::new();

    for i in 0..kmers.len() {
        match i {
            0 => string.push_str(kmers.get(i).unwrap()),
            _ => string.push(kmers.get(i).unwrap().chars().nth(k-1).unwrap()),
        }
    }

    return string;
}

pub fn run(content: Vec<String>) {
    let k_d : Vec<usize>= content.get(0).unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let (k, d) = (k_d[0], k_d[1]);

    let mut pref_vec: Vec<String> = Vec::new();
    let mut suff_vec: Vec<String> = Vec::new();

    for i in 1..content.len() {
        let kmer = VirtualKmer::new(&content[i]);
        pref_vec.push(kmer.first);
        suff_vec.push(kmer.second);
    }

    let pref_string = stringSpelledByPatterns(&pref_vec, k);
    let suff_string = stringSpelledByPatterns(&suff_vec, k);

    for i in k+d..pref_string.len() {
        if pref_string.chars().nth(i) != suff_string.chars().nth(i-k-d) {
            println!("there is no string spelled by the gapped patterns");
            break
        }
    }

    let mut concat_string = pref_string;
    concat_string.push_str(&suff_string[&suff_string.len()-k-d..]);
    println!("{concat_string:?}");
}