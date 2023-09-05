use std::collections::HashSet;

pub fn run(content : &Vec<String>) {
    let k = content[0].parse::<u8>().unwrap();
    let text = &content[1];

    let mut kmer_map: HashSet<&str> = HashSet::new();
    for i in 0..=text.len()-k as usize {
        kmer_map.insert(&text[i..i+k as usize]);
    }

    for kmer in kmer_map {
        println!("{kmer}");
    }
}