/*
 * File: D.rs
 * Project: BA4
 * File Created: 9th Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Compute the Number of Peptides of Given Total Mass */
use crate::biology::aminoacid::AMINOACID_MASS;
use std::collections::HashMap;

fn factorial(n:u128) -> u128 {
    let mut cases = 1;

    for i in 1..=n {
        cases *= i;
    }
    cases
}

fn bruteforce(max_idx: usize, mass: usize, total: &mut u128,
            cpy_vec: Vec<usize>, mass_vec: &Vec<usize>, //mass2aa: &HashMap<usize, usize>
        ) {
    let max_mass = *mass_vec.get(max_idx).unwrap();
    if max_mass > mass {
        return
    }
    let max_cpy = mass / max_mass;

    let mut cpy_update = cpy_vec.clone();
    for n in 1..=max_cpy {
        let remainder = mass - (max_mass * n);
        cpy_update[max_idx] = n;

        if (remainder == 0) {
            let sum: u128 = cpy_update.iter().sum::<usize>().try_into().unwrap();
            let mut cases: u128 = factorial(sum);
            for idx in 0..cpy_update.len() {
                let cpy: usize = cpy_update[idx];
                if cpy>0 {
                    let cpy_ :u128= u128::try_from(cpy).unwrap();
                    cases /= factorial(cpy_);
                    // NOTE: ParentMass(Spectrum) Only check spectrum. No need to count different types of AA
                    // let aa = *mass2aa.get(&mass_vec[idx]).unwrap();
                    // if aa > 1 {
                    //     let aa: u128 = aa.try_into().unwrap();
                    //     cases *= aa.pow(cpy_.try_into().unwrap());
                    // }
                }
            }
            *total += cases;
            return
        }

        for idx in 0..max_idx {
            bruteforce(idx, remainder, total, cpy_update.clone(), mass_vec); //, mass2aa);
        }
    }
}


pub fn run(content: Vec<String>) {
    let m = content[0].parse::<usize>().unwrap();

    let mut mass2aa: HashMap<usize, usize> = HashMap::new();
    let mut mass_uniq: Vec<usize> = Vec::new();
    for (aa,m) in &AMINOACID_MASS {
        if !mass2aa.contains_key(m) {
            mass_uniq.push(*m);
            mass2aa.insert(*m, 0);
        }
        // mass2aa.entry(*m).and_modify(|x| *x+=1);
    }

    mass_uniq.sort();

    let mut total = 0;

    for max_idx in 0..mass_uniq.len() {
        let init = vec![0;mass_uniq.len()];
        bruteforce(max_idx, m, &mut total, init, &mass_uniq) //, &mass2aa);
    }
    println!("{total}");
}