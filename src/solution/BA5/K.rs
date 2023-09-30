/* Find a Middle Edge in an Alignment Graph in Linear Space */
use crate::biology::aminoacid::ScoreMatrix;
use std::cmp;

static PENALTY: i32= -5;

pub fn compute_linear(v:&String, wi:char, score: &mut Vec<i32>) {
    let blosum = ScoreMatrix::build_blosum62();
    score[0] += PENALTY;

    for (i, vi) in v.chars().enumerate() {
        let left = score[i] + PENALTY;
        let up = score[i+1] + PENALTY;
        let diagonal = score[i] + blosum.matrix.get(&(wi,vi)).unwrap();
        score[i+1] = [left, up, diagonal].into_iter().max().unwrap();
    }
}

pub fn initialize_vec(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        vec[i] = i as i32 *PENALTY;
    }
}

pub fn find_middle(v:&String, w: &String) ->(usize,usize) {
    let n = v.len();
    let m = w.len();
    let (mut x,y) = (0, m/2);

    // From source to (i,m/2)
    let mut from_src : Vec<i32> = vec![0;n+1];
    initialize_vec(&mut from_src);
    for (i,wi) in w[..y].chars().enumerate() {
        compute_linear(&v, wi, &mut from_src);
    }

    let mut to_sink : Vec<i32> = vec![0;n+1];
    let v_rev: String = v.chars().rev().collect();
    initialize_vec(&mut to_sink);
    for (i, wi) in w[y..].chars().rev().enumerate() {
        compute_linear(&v_rev, wi, &mut to_sink);
    }
    to_sink = to_sink.into_iter().rev().collect();

    let blosum = ScoreMatrix::build_blosum62();
    let wi = w[y..y+1].chars().nth(0).unwrap();
    for (i,vi) in v.chars().enumerate() {
        // from_src[i] = from_src[i]+to_sink[i+1]+blosum.matrix.get(&(wi,vi)).unwrap();
        from_src[i] = cmp::max(from_src[i]+to_sink[i]+PENALTY, from_src[i]+to_sink[i+1]+blosum.matrix.get(&(wi,vi)).unwrap());
    }
    from_src.pop();
    let max_val = from_src.iter().max().unwrap();
    let x = from_src.iter().position(|r| r == max_val).unwrap();

    (x,y)
}

pub fn run(content:&Vec<String>) {
    let v = &content[0];
    let w = &content[1];
    let (x,y) = find_middle(&v, &w);
    println!("({}, {}) ({}, {})", x, y, x+1, y+1);
}