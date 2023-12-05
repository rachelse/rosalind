/* Compute the Probability of an Outcome Given a Hidden Path */

use super::A;

pub fn HMM(x:&String, transition:&String, alphabets: &Vec<char>, states: &Vec<char>,  emission: &Vec<Vec<f32>>) -> f32 {
    let mut prob = 1.0;
    for (i, alphabet) in x.char_indices() {
        let emission_idx = A::state_to_idx(alphabets, alphabet);
        let transition_idx = A::state_to_idx(states, transition.chars().nth(i).unwrap());

        prob *= emission[transition_idx][emission_idx];
    }
    prob
}

pub fn run(content: Vec<String>) {
    let x = &content[0];
    let alphabets: Vec<char> = content[2].chars().filter(|&x| x!=' ' && x!='\t').collect();
    let hidden_path = &content[4]; 
    let states: Vec<char> = content[6].chars().filter(|&x| x!=' ' && x!='\t').collect(); 
    let mut emission_matrix = Vec::new();

    for i in 9..content.len() {
        emission_matrix.push(A::_parse_line(&content[i]));
    }
    let prob = HMM(x, hidden_path, &alphabets, &states, &emission_matrix);
    println!("{prob}");
}