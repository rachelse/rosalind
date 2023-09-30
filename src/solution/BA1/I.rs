use std::collections::HashMap;
use crate::{*,
            biology::kmer::*};


#[derive(Debug)]
pub struct Data {
    text: String,
    k: u8,
    d: u8
}

impl Data {
    pub fn get(content: &Vec<String>) -> Data {
        let text = content.get(0).unwrap().clone();
        let mut k_d = content.get(1).unwrap().split_whitespace();
        let k = k_d.next().unwrap().parse::<u8>().unwrap();
        let d = k_d.next().unwrap().parse::<u8>().unwrap();
        Data { text, k, d }
    }
}


pub fn solve(data: &Data) -> Vec<String> {
    let kmer_list = Kmer::new(data.k);
    let mut kmer_count = HashMap::new();

    let mut max_count = 0;
    let mut max_kmer : Vec<String> = Vec::new();


    for kmer in &kmer_list.kmer {
        let mut tmp_count = 0;
        for i in 0..data.text.len()-data.k as usize {
            let tmp = &data.text[i..i+data.k as usize];
            if calc_hammdist(&kmer, tmp) <= data.d {
                tmp_count +=1;
            }
        }

        kmer_count.insert(kmer.as_str(), tmp_count);

        match tmp_count {
            cnt if cnt == max_count => max_kmer.push(kmer.clone()),
            cnt if cnt > max_count => {max_kmer.clear();
                                            max_count = tmp_count;
                                            max_kmer.push(kmer.clone())},
            _ => ()
        }
    }
    max_kmer
}

pub fn run(content: &Vec<String>) {
    let data = Data::get(content);
    
    solve(&data);
    let ans = solve(&data);
    
    print_answer(ans);
}