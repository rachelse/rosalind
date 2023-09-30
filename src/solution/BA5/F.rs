/* Find a Highest-Scoring Local Alignment of Two Strings */

use crate::biology::aminoacid::ScoreMatrix;
static PENALTY: i32 = -5;

#[derive(Clone, Debug)]
pub enum Direction {
    Right,
    Down,
    Diagonal,
    None,
}

pub fn align(s1: &String, s2: &String) {
    let pam250 = ScoreMatrix::build_pam250();
    let mut score = _initialize_matrix(s1.len(), s2.len());
    let mut direction = vec![vec![Direction::None;s1.len()+1];s2.len()+1];
    let mut track_position = (0,0);
    let mut score_max = 0;

    for i2 in 1..=s2.len() {
        for i1 in 1..=s1.len() {
            let a1 = s1[i1-1..i1].chars().next().unwrap();
            let a2 =  s2[i2-1..i2].chars().next().unwrap();
            let blosum_score = score[i2-1][i1-1] + pam250.matrix[&(a1,a2)] as i32;
            let left = score[i2][i1-1] + PENALTY;
            let upper = score[i2-1][i1] + PENALTY;

            let arg_max = [left, upper, blosum_score, 0_i32].into_iter().max().unwrap();
            
            match Some(arg_max) {
                _ if left == arg_max => {score[i2][i1] = left; direction[i2][i1] = Direction::Right},
                _ if upper == arg_max => {score[i2][i1] = upper; direction[i2][i1] = Direction::Down},
                _ if blosum_score == arg_max => {score[i2][i1] = blosum_score; direction[i2][i1] = Direction::Diagonal},
                _ => {score[i2][i1] = 0; direction[i2][i1] = Direction::None}
            }
            if arg_max > score_max {
                score_max = arg_max;
                track_position = (i1,i2);
            }
        }
    }
    println!("{}", score_max);
    backtrack(&s1, &s2, &score,&direction, &track_position);

}


pub fn backtrack(s1:&String, s2:&String, score:&Vec<Vec<i32>>, direction:&Vec<Vec<Direction>>, start:&(usize,usize)) {
    let (mut idx1,mut idx2) = start;

    let mut align1_rev = String::new();
    let mut align2_rev = String::new();
    
    while idx1>0 || idx2>0 {
        if score[idx2][idx1] <= 0 {break}
        match direction[idx2][idx1] {
            Direction::Right => {align1_rev.push_str(&s1[idx1-1..idx1]);align2_rev.push('-');idx1-=1;},
            Direction::Down => {align1_rev.push('-');align2_rev.push_str(&s2[idx2-1..idx2]);idx2-=1;},
            Direction::Diagonal => {align1_rev.push_str(&s1[idx1-1..idx1]);align2_rev.push_str(&s2[idx2-1..idx2]);idx1-=1;idx2-=1;},
            _ => {break }
        }
    }

    let align1 : String= align1_rev.chars().rev().collect();
    let align2 : String = align2_rev.chars().rev().collect();

    println!("{align1}");
    println!("{align2}");
}


pub fn _initialize_matrix(len1:usize, len2:usize) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> =vec![vec![0;len1+1];len2+1];
    return matrix
}


pub fn run(content: &Vec<String>) {
    let s1 = &content[0];
    let s2 = &content[1];
    align(s1,s2);
}