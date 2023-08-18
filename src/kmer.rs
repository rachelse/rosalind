use std::collections::HashSet;
use crate::nucleotide;

pub struct Kmer {
    pub k : u8,
    pub kmer : HashSet<String>
}

impl Kmer {
    pub fn new(k: u8) -> Self {
        let mut kmer = Kmer { k: k,
                                kmer: HashSet::new()};
        Self::get_all_kmers(&mut kmer);
        kmer
    }

    fn get_all_kmers(kmer: &mut Self) {
        let nt = nucleotide::NUCLEOTIDE;

        let mut kmer_vec: Vec<String> = Vec::new();

        for nt in nucleotide::NUCLEOTIDE {
            kmer_vec.push(nt.to_string());
        }
        let mut left = kmer.k - 1;

        while left > 0 {
            kmer_vec = Self::fill_kmer(kmer_vec, left);
            left -= 1;
        }
        
        for k in kmer_vec {
            kmer.kmer.insert(k);
        }
    }

    fn fill_kmer(kmer_vec: Vec<String>, k: u8) -> Vec<String>{
        if k <= 0 {
            return kmer_vec
        } else {
            let mut filled = Vec::new();
            for kmer in kmer_vec.iter() {
                for nt in nucleotide::NUCLEOTIDE {
                    filled.push(format!("{}{}",kmer,nt));
                }
            }
            filled
        }
    }

}

pub fn calc_hammdist(k1: &str, k2: &str) -> u8 {
    let mut dist:u8 = 0;
    for (c1, c2) in k1.chars().zip(k2.chars()) {
        if c1 != c2 { dist += 1;}
    }
    dist
}