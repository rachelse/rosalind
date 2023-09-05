use std::collections::HashMap;
use crate::print_answer;

pub fn run(content: &Vec<String>) {
    let seq = &content[0];
    let k = &content[1].parse::<usize>().unwrap();

    let mut kmer_map: HashMap<&str,u8> = HashMap::new();

    for i in 0..=seq.len()-k {
        let kmer = &seq[i..i+k];
        let cnt = kmer_map.entry(kmer).or_insert(0);
        *cnt += 1;
    }

    let mut max_cnt = 0; 
    let mut word = Vec::new();
    for (kmer,cnt) in kmer_map {
        if cnt > max_cnt {
            max_cnt = cnt;
            word.clear();
            word.push(kmer);
        } else if cnt == max_cnt {
            word.push(kmer)
        }
    }

    print_answer(word);
}