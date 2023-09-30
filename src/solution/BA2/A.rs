use crate::{biology::kmer::{*, self}, print_answer};

pub struct Data {
    k : u8,
    d : u8,
    dna : Vec<String>,
}

impl Data {
    pub fn parse(content:&Vec<String>) -> Self {
        let mut k_d = content.get(0).unwrap().split_whitespace();
        let mut seq = Vec::new();
        for i in 1..content.len() {
            seq.push(content[i].clone());
        } 
        Data {
            k :k_d.next().unwrap().parse::<u8>().unwrap(),
            d: k_d.next().unwrap().parse::<u8>().unwrap(),
            dna : seq
        }
    }    
}

pub fn solve(data: &Data) -> Vec<String> {
    let kmer_set = Kmer::new(data.k);
    let mut motif = Vec::new();

    for kmer in &kmer_set.kmer {
        let mut total = 0;
        for seq in &data.dna {
            let mut exist = 0;
            let mut i = 0;
            while exist == 0 && i < seq.len() +1 -data.k as usize{
                if calc_hammdist(kmer.as_str(), &seq[i..i+data.k as usize]) <= data.d {
                    exist += 1;
                }
                i+=1;
            }

            if exist == 0 {
                break
            } else {
                total += exist;
            }
        }
        if total == data.dna.len() {
            motif.push(kmer.clone());
        }
    }
    motif 
}

pub fn run(content : &Vec<String>) {
    let data = Data::parse(content);
    let ans = solve(&data);

    print_answer(ans);

}