/* Construct the De Bruijn Graph of a String */
use std::collections::BTreeMap;

pub fn run(content: &Vec<String>) {
    let k = content[0].parse::<usize>().unwrap();
    let text = &content[1];

    let mut debruijn: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for i in 0..=text.len()-k {
        let kmer = &text[i..i+k-1];
        let adjacent = &text[i+1..i+k];
        let kmer_vec = debruijn.entry(kmer).or_insert(vec![]);
        kmer_vec.push(adjacent);
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