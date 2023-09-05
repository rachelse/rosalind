use rand::{Rng, random};
use std::time::{self, Instant};
use crate::kmer::*;
pub struct Data {
    k: u32,
    t: u32,
    n: u32,
    pub DNA : Vec<String>
}
impl Data {
    pub fn parse(content: &mut Vec<String>) -> Self {
        let k_t_n: Vec<u32> = content.get(0).unwrap().split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap()).collect();
        content.remove(0);
        let dna = content.to_owned();
        Data {
            k: k_t_n[0],
            t: k_t_n[1],
            n: k_t_n[2],
            DNA: dna
        }
    }
}

pub struct BestMotifs<'T>{
    pub best_motifs :Vec<&'T str>,
    pub score: u32
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

pub fn randomize_motifs<'T>(data:&'T Data) -> Vec<&'T str> {
    let mut motifs = Vec::new();
    for seq in data.DNA.iter() {
        let mut rng = rand::thread_rng();
        let n : usize= rng.gen_range(0..=data.DNA[0].len()-data.k as usize);
        motifs.push(&seq[n..n+data.k as usize]);
    }
    motifs
}

pub fn randomize_number(t: u32) -> usize {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..t as usize);
    i
}

pub fn build_profile(motifs:&Vec<&str>, k: u32, t:u32) -> Vec<[f32;4]>{
    let mut cnt : Vec<[u32;4]> = vec![[1_u32;4];k as usize];
    let mut profile = vec![[0.0_f32;4];k as usize];
    let mut excluded = false;
    for i in 0..t as usize {
        let motif = motifs[i];
        for j in 0..k as usize {
            if motif.len() == 0 {
                excluded = true;
                continue
            }
            let idx = nucleotide_to_num(&motif[j..j+1]).expect("expect index");
            cnt[j][idx] += 1;
        }
    }

    for i in 0..k as usize{
        for j in 0..4_usize {
            if excluded {
                let p = cnt[i][j] as f32 / (4+t-1) as f32;
                profile[i][j] = p;
            } else {
                let p = cnt[i][j] as f32 / (4+t) as f32;
                profile[i][j] = p;
            }
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

pub fn calc_prob(profile:&Vec<[f32;4]>, seq: &str) -> f32 {
    let mut prob = 1.0_f32;
    for (i,nt) in seq.chars().enumerate() {
        let j = nucleotide_to_num(nt.to_string().as_str()).unwrap();
        prob *= profile[i][j];
    }
    prob
}

pub fn find_motif<'T>(seq : &'T String, profile: &Vec<[f32;4]>) -> &'T str{
    let k = profile.len();
    let mut max_prob = 0.0_f32;
    let mut idx = 0_usize;
    for i in 0..=seq.len()-k {
        let tmp = &seq[i..i+k];
        let prob = calc_prob(profile, tmp);
        if prob > max_prob {
            max_prob = prob ;
            idx = i;
        }
    }
    return &seq[idx..idx+k]
}

pub fn update_motifs<'T>(profile:&Vec<[f32;4]>, dna: &'T Vec<String>, idx: usize) -> &'T str{
    let seq = &dna[idx];
    let motif = find_motif(&seq,  profile);

    motif
}

pub fn calc_score(profile:&Vec<[f32;4]>, motifs: &Vec<&str>) -> u32 {
    let consensus = get_consensus(profile);
    let mut dist = 0_u32;
    for motif in motifs {
        dist += calc_hammdist(motif, consensus.as_str()) as u32
    }
    dist
}

pub fn execute_turn(data : &Data) -> BestMotifs{
    let mut motifs = randomize_motifs(&data);
    
    let mut best_motifs = motifs.clone();
    let mut best_profile = build_profile(&best_motifs, data.k, data.t);
    let mut best_score = calc_score(&best_profile, &best_motifs);

    for j in 0..data.n as usize {
        let i = randomize_number(data.t);
        motifs[i] = "";

        let excluded_profile = build_profile(&motifs, data.k, data.t);
        let updated = update_motifs(&excluded_profile, &data.DNA, i);
        motifs[i] = updated;

        let profile = build_profile(&motifs, data.k, data.t);
        let score = calc_score(&profile, &motifs);

        if score < best_score {
            best_motifs = motifs.clone();
            best_score = score;
        }
    }

    BestMotifs {
        best_motifs: best_motifs,
        score: best_score
    }
}

pub fn solve<'T>(data: &'T Data) -> Vec<&'T str> {
    let mut turn = 0;
    let mut best = BestMotifs { 
                    best_motifs : Vec::new(), 
                    score : u32::MAX};
    while turn < 100 {
        println!("turn {turn}");
        let tmp = execute_turn(data);
        if tmp.score < best.score {
            best = tmp;
        }
        turn +=1;
    }
    best.best_motifs
}

pub fn run(content : &mut Vec<String>) {
    let now = Instant::now();
    let data = Data::parse(content);
    let ans = solve(&data);
    for line in ans {
        println!("{line}");
    }
    let elapsed = now.elapsed();
    println!("took {}sec", elapsed.as_secs());
}