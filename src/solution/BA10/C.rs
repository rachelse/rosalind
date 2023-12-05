/* Implement the Viterbi Algorithm */
use super::A;
use crate::utils::float::*;
pub fn _parse_line(line : &String) -> Vec<f64> {
    let splitted: Vec<f64> = line.split_whitespace().skip(1).map(|x| x.parse::<f64>().unwrap()).collect();
    splitted
}

pub fn decode(score : Vec<Vec<f64>>, states: &Vec<char>, tracker: &Vec<Vec<usize>>) -> String {
    let mut path_rev = String::new();
    let mut last_idx = get_max_f64(score.last().unwrap());
    path_rev.push(A::idx_to_state(states, last_idx));

    // Track
    for prev in tracker.iter().skip(1).rev(){
        last_idx = prev[last_idx];
        path_rev.push(A::idx_to_state(states, last_idx));
    }
    
    path_rev.chars().rev().collect()
}

pub fn Viterbi(x: &String, hidden: &Vec<char> , states: &Vec<char>, transition: &Vec<Vec<f64>>, emission: &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
    let mut path_score: Vec<Vec<f64>> = Vec::new();
    let mut tracker : Vec<Vec<usize>> = Vec::new();

    for (i, h) in x.char_indices() {
        let mut weight = vec![1.0;states.len()];
        let mut track = vec![0;states.len()];

        // Compute emission
        let h_idx = A::state_to_idx(&hidden, h);
        for (l, emission_vec) in emission.iter().enumerate() {
            weight[l] *= emission_vec[h_idx];
        }

        // Compute transition
        if i != 0 {
            let last = &path_score.last().unwrap();

            for ith in 0..weight.len() {
                let mut max_ith = 0.0;
                let mut max_idx = 4;

                for (ith_1, prev) in last.iter().enumerate() {
                    let tmp = prev * transition[ith_1][ith];
                    if tmp > max_ith {
                        max_ith = tmp;
                        max_idx = ith_1;
                    }
                }

                weight[ith] *= max_ith;
                track[ith] = max_idx;
            }
        }
        path_score.push(weight);
        tracker.push(track);
    }

    (path_score, tracker)
}

pub fn run(content: Vec<String>) {
    let x = &content[0];
    let alphabets: Vec<char> = content[2].chars().filter(|&x| x!=' ' && x!='\t').collect();
    let states: Vec<char> = content[4].chars().filter(|&x| x!=' ' && x!='\t').collect(); 
    let mut transition = Vec::new();
    let mut emission = Vec::new();

    for i in 7..7+states.len() {
        transition.push(_parse_line(&content[i]));
    }

    for i in 9+states.len()..content.len() {
        emission.push(_parse_line(&content[i]));
    }
    let (score,tracker)= Viterbi(x, &alphabets, &states, &transition, &emission);
    let path = decode(score, &states, &tracker);
    println!("{path}");
}