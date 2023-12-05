/* Compute the Probability of a String Emitted by an HMM */
use super::A;
use crate::utils::float;

pub fn _parse_line(line : &String) -> Vec<f64> {
    let splitted: Vec<f64> = line.split_whitespace().skip(1).map(|x| x.parse::<f64>().unwrap()).collect();
    splitted
}

pub fn Viterbi(x: &String, hidden: &Vec<char> , states: &Vec<char>, transition: &Vec<Vec<f64>>, emission: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut forward: Vec<Vec<f64>> = Vec::new();

    for (i, h) in x.char_indices() {
        let mut weight = vec![0.0;states.len()];

        // Compute transition
        if i != 0 {
            let last = &forward.last().unwrap();

            for ith in 0..weight.len() {
                for (ith_1, prev) in last.iter().enumerate() {
                    weight[ith] +=prev * transition[ith_1][ith]; 
                }
            }
        } else {
            for w in weight.iter_mut() {
                *w += (1.0/states.len() as f64);
            }
        }

        // Compute emission
        let h_idx = A::state_to_idx(&hidden, h);
        for (l, emission_vec) in emission.iter().enumerate() {
            weight[l] *= emission_vec[h_idx];
        }
        forward.push(weight);
    }

    forward
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
    let forward= Viterbi(x, &alphabets, &states, &transition, &emission);

    println!("{:?}", forward.last().unwrap().iter().sum::<f64>());
}