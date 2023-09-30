/* Construct the De Bruijn Graph of a Collection of k-mers */
use std::collections::BTreeMap;

pub fn run(content: &Vec<String>) {
    let mut debruijn: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for kmer in content {
        let k = kmer.len();
        let kmer_vec = debruijn.entry(&kmer[..k-1]).or_insert(vec![]);
        kmer_vec.push(&kmer[1..]);
        kmer_vec.sort();
    }

    for (k,v) in debruijn.into_iter() {
        let mut v_iter = v.iter();
        print!("{k} -> {}", v_iter.next().unwrap());
        for a in v_iter {
            print!(",{a}");
        }
        println!();
    }
}