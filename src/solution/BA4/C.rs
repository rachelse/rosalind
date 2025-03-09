/*
 * File: C.rs
 * Project: BA4
 * File Created: 9th Mar 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */
/* Generate the Theoretical Spectrum of a Cyclic Peptide */
use crate::{biology::aminoacid::AMINOACID_MASS, print_answer};

pub fn calcMass(peptide : &str) -> usize {
    let mut mass = 0;
    for mut aa in  peptide.chars() {
        mass += *AMINOACID_MASS.get(&aa).unwrap();
    }
    mass
}
pub fn run(content: Vec<String>) {
    let peptide = &content[0];
    let peptide2 = format!("{peptide}{peptide}");

    let mut cyclospectrum: Vec<usize> = vec![0];
    for i in 0..peptide.len() {
        for sublen in 1..peptide.len() {
            let subpeptide = &peptide2[i..i+sublen];
            let mass = calcMass(subpeptide);
            cyclospectrum.push(mass);
        }
    }
    let sum: usize = calcMass(&peptide);
    cyclospectrum.push(sum);

    cyclospectrum.sort();
    print_answer(cyclospectrum);
}