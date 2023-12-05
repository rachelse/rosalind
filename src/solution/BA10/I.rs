/* Implement Viterbi Learning */
use super::{C,H};

fn update_path(x: &String, alphabet: &Vec<char>, states: &Vec<char>, transition: &Vec<Vec<f64>>, emission: &Vec<Vec<f64>>)-> String{
    let (score, tracker) = C::Viterbi(x, &alphabet, &states, &transition, &emission);
    let path = C::decode(score, &states, &tracker);
    path
}

fn update_parameter(x: &String, path: &String, states: &Vec<char>, alphabet: &Vec<char>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let transition = H::transition(path, states);
    let emission = H::emission(x, path, alphabet, states);
    (transition, emission)
}

pub fn run(content: Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    let x = &content[2];
    let alphabet : Vec<char> = content[4].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();
    let states : Vec<char> = content[6].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();

    let mut transition : Vec<Vec<f64>> = Vec::new();
    for i in 9..9+states.len() {
        transition.push(C::_parse_line(&content[i]));
    }

    let mut emission : Vec<Vec<f64>> = Vec::new();
    for i in 11+states.len()..content.len() {
        emission.push(C::_parse_line(&content[i]));
    }

    for i in 0..n {
        let path = update_path(x, &alphabet, &states, &transition, &emission);
        (transition, emission) = update_parameter(x, &path, &states, &alphabet);
    }

    H::print_matrix(&transition, &states, &states);
    println!("--------");
    H::print_matrix(&emission, &states, &alphabet);
}