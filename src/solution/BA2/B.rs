use crate::biology::kmer::*;

pub struct Data {
    k : u8,
    dna : Vec<String>,
}

impl Data {
    pub fn parse(content:&Vec<String>) -> Self {
        let mut k = content.get(0).expect("Failed to get first line")
                                .parse::<u8>().expect("k is not parsed into u8");
        let mut seq = Vec::new();
        for i in 1..content.len() {
            seq.push(content[i].clone());
        } 
        Data {
            k :k,
            dna : seq
        }
    }    
}

pub fn solve(data: &Data) -> String {
    let kmer_set = Kmer::new(data.k);
    let mut min_kmer = String::new();
    let mut min_dist = data.k * data.dna.len() as u8;
    // let mut min_vec = Vec::new();

    for kmer in &kmer_set.kmer {
        let mut total_dist = 0;
        for seq in &data.dna {
            let mut tmp_min = data.k;
            let mut idx = 0_usize;
            while (idx <= seq.len()-data.k as usize) && tmp_min > 0{

                let tmp_dist = calc_hammdist(kmer.as_str(), &seq[idx..idx+data.k as usize]);
                if tmp_dist < tmp_min {
                    tmp_min = tmp_dist;
                }
                idx+=1;
            }
            total_dist += tmp_min;
        }
        if kmer == &"GAC".to_string() {
        }
        if min_dist >= total_dist {
            min_dist = total_dist;
            min_kmer = kmer.clone();
            // min_vec.push(kmer.clone());
        }

    }

    min_kmer

}

pub fn run(content : &Vec<String>) {
    let data = Data::parse(content);
    let ans = solve(&data);

    println!("{}",ans);
}