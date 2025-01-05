/*
 * File: B.rs
 * Project: BA4
 * File Created: 5th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

/* Find Substrings of a Genome Encoding a Given Amino Acid String */
use crate::biology::aminoacid::DNA_CODON1;

enum strand {
    PLUS,
    MINUS
}

fn get_reverse(origin: &String) -> String {
    let mut reverse_complement: String = String::new();
    for nt in origin.chars() {
        match nt {
            'A' => reverse_complement.push('T'),
            'T' => reverse_complement.push('A'),
            'C' => reverse_complement.push('G'),
            'G' => reverse_complement.push('C'),
            _ => continue
        }
    }
    reverse_complement = reverse_complement.chars().rev().collect();
    return reverse_complement
}

fn find_substring(dna: &String, peptide: &String, strand : strand) {
    let codon_len = peptide.len()*3;

    // let dna_reverse = get_reverse(content.first().unwrap());

    // iter over frame
    for frame in 0..3 {
        let mut start = frame;
        // check each nucleotide seq. in frame
        while (start + codon_len) < dna.len() {
            let tmp_nt = &dna[start..start+codon_len];
            let mut tmp_aa = String::new();
            for i in (0..codon_len).step_by(3) {
                tmp_aa.push(*DNA_CODON1.get(&tmp_nt[i..i+3]).unwrap());
            }
            
            // check if the aa is identical to the given peptide
            if &tmp_aa == peptide {
                match strand {
                    strand::PLUS => println!("{tmp_nt}"),
                    strand::MINUS => println!("{}", get_reverse(&tmp_nt.to_string())),
                    _ => continue
                }

            }
            start += 3;
        }
    }
}

pub fn run(content: Vec<String>) {
    let dna = content.first().unwrap();
    let peptide = content.last().unwrap();

    let reverse_complement = get_reverse(content.first().unwrap());

    find_substring(dna, peptide, strand::PLUS);
    find_substring(&reverse_complement, peptide, strand::MINUS); 
}