use crate::{biology::kmer::*, print_answer};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Data {
    text: String,
    k : u8,
}

impl Data {
    pub fn build(content: &Vec<String>) -> Self {
        let text = content.get(0).unwrap().clone();
        let k = content.get(1).unwrap().parse::<u8>().unwrap();
        Data {text, k}
    }
}

pub fn solve(data: &Data) -> Vec<i32> {
    let kmer_set = Kmer::new(data.k);

    let mut kmer_map = BTreeMap::new();

    for kmer in &kmer_set.kmer {
        kmer_map.insert(kmer.as_str(),0);
    }

    for i in 0..data.text.len()-data.k as usize +1 {
        *kmer_map.get_mut(&data.text[i..i+data.k as usize]).unwrap() += 1;
    }

    let mut cnts = Vec::new();
    for (kmer,cnt) in kmer_map {
        cnts.push(cnt);
    }

    cnts
}

pub fn run(content : &Vec<String>) {
    let data = Data::build(content);
    let ans = solve(&data);
    print_answer(ans);
}