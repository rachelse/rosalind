use crate::{*,
            nucleotide::*, 
            mathematics::stats::*};

fn get_variation(pattern: &str, idx: &Vec<u8>) -> Vec<String> {
    // let num = 3_u32.pow(idx.len() as u32);
    // let neighbors = Vec::with_capacity(num as usize);
    
    let mut check: Vec<i32> = vec![0;pattern.len()];
    for i in idx {
        check[*i as usize] = 1;
    }

    let mut neighbors = Vec::new();
    if let 0 = check[0] {
        neighbors.push(pattern[0..1].to_string());
    } else {
        let mut nucleotide = vec![String::from("A"), String::from("C"), String::from("G"), String::from("T")];
        nucleotide.retain(|x| x.as_str() != &pattern[0..1]);
        neighbors.extend(nucleotide);
    }

    for i in 1..check.len() {
        if let 0 = check[i] {
            for n in 0..neighbors.len() {
                neighbors[n] = format!("{}{}", neighbors[n], &pattern[i..i+1]);
            }
        } else {
            let mut nucleotide = vec![String::from("A"), String::from("C"), String::from("G"), String::from("T")];
            nucleotide.retain(|x| x.as_str() != &pattern[i..i+1]);
            neighbors = neighbors.iter().map(|a| nucleotide.iter()
                            .map(move |b| format!("{a}{b}"))).flatten().collect();
        }
    }

    neighbors
}

pub fn solve(pattern: &str, d:u8) -> Vec<String> {
    let length = pattern.len() as u8;
    let mut pattern_vec = vec![pattern.to_owned()]; 

    // Find all the neighbors sand fill the pattern_vec
    for i in 1..=d as usize {
        let mut dist_variations = Combination::new(length, i as u8);
        dist_variations.fill_combination();
  
        for variation in &dist_variations.combination {
            let neighbors = get_variation(pattern, &variation);
            pattern_vec.extend(neighbors);
        }
    }

    pattern_vec
}

pub fn run(content : &Vec<String>) {
    let pattern : &str= content[0].as_str();
    let d : u8 = content[1].parse().expect("String cannot be parsed into number");
    let ans = solve(pattern, d);
    for a in ans {
        println!("{a}");
    }
}