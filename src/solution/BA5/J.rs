/* Align Two Strings Using Affine Gap Penalties */

use crate::biology::aminoacid::ScoreMatrix;

static OPENING : i32 = -11;
static EXTENSION : i32 = -1;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Track{
    None,
    Left,
    Up,
    Diagonal,
}

pub fn fill_lower(v:usize, w:usize, lower: &Vec<Vec<i32>>, middle: &Vec<Vec<i32>>) -> (i32,Track) {
    // lower: up to down
    let from_lower = lower[w-1][v]+EXTENSION;
    let from_middle = middle[w-1][v]+OPENING;

    if from_lower >= from_middle  {
        return (from_lower,Track::Up)
    } else {
        return (from_middle, Track::Diagonal)
    }
}

pub fn fill_upper(v:usize, w:usize, middle: &Vec<Vec<i32>>, upper: &Vec<Vec<i32>>) -> (i32,Track) {
    // upper: left to right
    let from_middle = middle[w][v-1]+OPENING;
    let from_upper = upper[w][v-1]+EXTENSION;
    if from_upper >= from_middle {
        return (from_upper, Track::Left)
    } else {
        return (from_middle, Track::Diagonal)
    }
}

pub fn fill_middle(v:usize, w:usize, lower: &Vec<Vec<i32>>, middle: &Vec<Vec<i32>>, upper: &Vec<Vec<i32>>, blosum:i32) -> (i32,Track) {
    // middle: diagonal (match or mismatch)
    let from_lower = lower[w][v];
    let from_upper = upper[w][v];
    let from_middle = middle[w-1][v-1]+blosum;
    let (max_idx,max_val) = [from_lower, from_upper, from_middle].into_iter().enumerate()
                        .max_by(|(_,a),(_,b)| a.cmp(b)).unwrap();
    match Some(max_idx) {
        Some(0) => {return (from_lower, Track::Up)},
        Some(1) => {return (from_upper, Track::Left)},
        _ => {return (from_middle, Track::Diagonal)}
    }
}

pub fn fit_alignment(v:&String, w: &String, lower: &mut Vec<Vec<i32>>, middle: &mut Vec<Vec<i32>>, upper: &mut Vec<Vec<i32>>) {
    let lenv = v.len();
    let lenw = w.len();
    let blosum62 = ScoreMatrix::build_blosum62();
    let mut track_lower : Vec<Vec<Track>> = vec![vec![Track::None;lenv+1];lenw+1];
    let mut track_middle : Vec<Vec<Track>> = vec![vec![Track::None;lenv+1];lenw+1];
    let mut track_upper : Vec<Vec<Track>> = vec![vec![Track::None;lenv+1];lenw+1];

    for iv in 1..=lenv {
        for iw in 1..=lenw {
            let from_lower = fill_lower(iv, iw, &lower, &middle);
            lower[iw][iv] = from_lower.0;
            track_lower[iw][iv] = from_lower.1;

            let from_upper = fill_upper(iv, iw, &middle, &upper);
            upper[iw][iv] = from_upper.0;
            track_upper[iw][iv] = from_upper.1;

            let blosum = blosum62.get_score(&w[iw-1..iw], &v[iv-1..iv]).unwrap();
            let from_middle = fill_middle(iv, iw, &lower, &middle, &upper, blosum);
            middle[iw][iv] = from_middle.0;
            track_middle[iw][iv] = from_middle.1;
        }
    }
    let (max_idx,max_score) = [middle[lenw][lenv],lower[lenw][lenv], upper[lenw][lenv]].into_iter().enumerate()
                        .max_by(|(_,a),(_,b)| a.cmp(b)).unwrap();
    
    println!("{max_score}");
    match max_idx {
        x if x == 0 => {track(&v, &w, Track::Diagonal, &track_lower, &track_middle, &track_upper)},
        x if x == 1 => {track(&v, &w, Track::Up, &track_lower, &track_middle, &track_upper)},
        x if x == 2 => {track(&v, &w, Track::Left, &track_lower, &track_middle, &track_upper)},
        _ => ()
    }
}

pub fn track( v:&String, w:&String, start:Track, lower: &Vec<Vec<Track>>, middle: &Vec<Vec<Track>>, upper: &Vec<Vec<Track>>) {
    let (mut vi,mut wi) = (v.len(), w.len());
    let mut aligned_v = String::new();
    let mut aligned_w = String::new();

    let mut track = &start;
    
    while vi>0 && wi>0 {
        if track == &Track::Diagonal {
            track = &middle[wi][vi];
            match track {
                Track::Diagonal => {aligned_v.push_str(&v[vi-1..vi]);aligned_w.push_str(&w[wi-1..wi]);wi-=1;vi-=1;}
                _ => (),
            }
        } else if track == &Track::Up {
            aligned_v.push('-');aligned_w.push_str(&w[wi-1..wi]);
            track = &lower[wi][vi];
            wi-=1;
        } else if track == &Track::Left {
            aligned_v.push_str(&v[vi-1..vi]);aligned_w.push('-');
            track = &upper[wi][vi];
            vi-=1;
        }
    }

    aligned_v = aligned_v.chars().rev().collect();
    aligned_w = aligned_w.chars().rev().collect();
    println!("{aligned_v}");
    println!("{aligned_w}");
}

pub fn run(content:&Vec<String>) {
    let v = &content[0];
    let w = &content[1];
    let lenv = v.len();
    let lenw = w.len();

    let mut lower: Vec<Vec<i32>> = vec![vec![0;lenv+1];lenw+1];
    let mut middle: Vec<Vec<i32>> = vec![vec![0;lenv+1];lenw+1];
    let mut upper: Vec<Vec<i32>> = vec![vec![0;lenv+1];lenw+1];

    fit_alignment(&v, &w, &mut lower, &mut middle, &mut upper);
}