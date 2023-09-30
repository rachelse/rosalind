use rand::{Rng, random};
use crate::biology::kmer::*;
pub struct Data {
    k: u8,
    t: u8,
    pub DNA : Vec<String>
}
impl Data {
    pub fn parse(content: &mut Vec<String>) -> Self {
        let k_t: Vec<u8> = content.get(0).unwrap().split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect();
        content.remove(0);
        let dna = content.to_owned();
        Data {
            k: k_t[0],
            t: k_t[1],
            DNA: dna
        }
    }
}

pub struct BestMotifs{
    pub best_motifs :Vec<String>,
    pub score: u8
}

pub fn num_to_nucleotide(num:usize) -> Option<char> {
    match num {
        0 => {return Some('A')},
        1 => {return Some('C')},
        2 => {return Some('G')},
        3 => {return Some('T')},
        _ => {return None}
    }
}

pub fn nucleotide_to_num(nt:&str) -> Option<usize> {
    match nt {
        "A" => {return Some(0)},
        "C" => {return Some(1)},
        "G" => {return Some(2)},
        "T" => {return Some(3)},
        _ => {return None}
    }
}

pub fn randomize_motifs<'T>(data:&'T Data) -> Vec<String> {
    let mut motifs = Vec::new();
    for seq in data.DNA.iter() {
        let mut rng = rand::thread_rng();
        let n : usize= rng.gen_range(0..=data.DNA[0].len()-data.k as usize);
        motifs.push(seq[n..n+data.k as usize].to_owned());
    }
    motifs
}

pub fn build_profile(motifs:&Vec<String>, k: u8, t:u8) -> Vec<[f32;4]>{
    let mut cnt : Vec<[u8;4]> = vec![[1_u8;4];k as usize];
    let mut profile = vec![[0.0_f32;4];k as usize];

    for i in 0..t as usize {
        let motif = motifs.get(i).unwrap();
        for j in 0..k as usize {
            let idx = nucleotide_to_num(&motif[j..j+1]).unwrap();
            cnt[j][idx] += 1;
        }
    }

    for i in 0..k as usize{
        for j in 0..4_usize {
            let p = cnt[i][j] as f32 / (4+t) as f32;
            profile[i][j] = p;
        }
    }

    profile
}

pub fn get_maxnt(profile_vec: &[f32;4]) -> char {
    let mut max = 0.0;
    let mut idx = 0;
    for (i, f) in profile_vec.into_iter().enumerate() {
        if f > &max {
            max = *f;
            idx = i;
        }
    }
    let nt = num_to_nucleotide(idx).unwrap();
    return nt
}

pub fn get_consensus(profile: &Vec<[f32;4]>) -> String {
    let mut consensus = String::new();
    for i in profile {
        consensus.push(get_maxnt(i));
    }
    consensus
}

pub fn find_motif<'T>(seq : &'T String, consensus: &String) -> &'T str{
    let k = consensus.len();
    let mut min_dist = k as u8;
    let mut idx = 0_usize;
    for i in 0..=seq.len()-k {
        let tmp = &seq[i..i+k];
        let dist = calc_hammdist(tmp, consensus.as_str());
        if dist < min_dist {
            min_dist = dist;
            idx = i
        }
    }
    return &seq[idx..idx+k]
}

pub fn update_motifs<'T>(profile:&Vec<[f32;4]>, dna: &'T Vec<String>) -> Vec<String>{
    let consensus = get_consensus(profile);
    let k = profile.len();
    let mut motifs = Vec::new();
    for seq in dna {
        let motif = find_motif(seq, &consensus);
        motifs.push(motif.to_owned());
    }
    motifs
}

pub fn calc_score(profile:&Vec<[f32;4]>, motifs: &Vec<String>) -> u8 {
    let consensus = get_consensus(profile);
    let mut dist = 0;
    for motif in motifs {
        dist += calc_hammdist(motif, consensus.as_str())
    }
    dist
}

pub fn execute_turn(data : &Data) -> BestMotifs{
    let mut motifs = randomize_motifs(&data);
    
    let mut best_motifs = motifs.clone();
    let mut best_profile = build_profile(&best_motifs, data.k, data.t);
    let mut best_score = calc_score(&best_profile, &best_motifs);

    while true {
        let profile = build_profile(&motifs, data.k, data.t);
        motifs = update_motifs(&profile, &data.DNA);
        let score = calc_score(&profile, &motifs);

        if score < best_score {
            best_motifs = motifs.clone();
            best_score = score;
        } else {
            break
        }
    }

    BestMotifs {
        best_motifs: best_motifs,
        score: best_score
    }
}

pub fn solve(data: &Data) -> Vec<String> {
    let mut turn = 0;
    let mut best = BestMotifs { 
                    best_motifs : Vec::new(), 
                    score : u8::MAX};
    while turn <= 1000 {
        let tmp = execute_turn(data);
        if tmp.score < best.score {
            best = tmp;
        }
        turn +=1;
        // println!("{turn} {:?}",best.best_motifs);
    }
    best.best_motifs
}

pub fn run(content : &mut Vec<String>) {
    let data = Data::parse(content);
    let ans = solve(&data);
    for line in ans {
        println!("{line}");
    }
}