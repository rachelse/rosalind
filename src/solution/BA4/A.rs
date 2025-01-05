/*
 * File: A.rs
 * Project: BA4
 * File Created: 5th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

 /* Translate an RNA String into an Amino Acid String */

use crate::biology::aminoacid::RNA_CODON1;

pub fn run(content: Vec<String>) {
    let rna = content.first().unwrap();
    let mut idx = 0;
    let mut aa = String::new();
    while (idx+2) < rna.len() {
        aa.push(*RNA_CODON1.get(&rna[idx..idx+3]).unwrap());
        idx+=3;
    }

    println!("{aa}");
}