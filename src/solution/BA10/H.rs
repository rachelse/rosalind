/* HMM Parameter Estimation Problem */
use super::{A,E};

pub fn transition(path: &String, states: &Vec<char>) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0;states.len()];states.len()];
    for (i,c) in path[..path.len()-1].char_indices() {
        let from = A::state_to_idx(states, c);
        let to = A::state_to_idx(states, path.chars().nth(i+1).unwrap() );
        matrix[from][to] += 1.0;
    }
    E::format_matrix(&mut matrix);
    for m in matrix.iter_mut() {
        if m.iter().sum::<f64>() == 0.0 {
            let val = 1.0 / m.len() as f64;
            for v in m {
                *v += val;
            }
        }
    }

    matrix
}

pub fn emission(text: &String, path: &String, alphabet: &Vec<char>, states: &Vec<char>) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0;alphabet.len()];states.len()];
    for(p,t) in path.chars().zip(text.chars()) {
        let from_p = A::state_to_idx(states,p);
        let to_t = A::state_to_idx(alphabet,t);
        matrix[from_p][to_t] += 1.0;
    }
    E::format_matrix(&mut matrix);
    for m in matrix.iter_mut() {
        if m.iter().sum::<f64>() == 0.0 {
            let val = 1.0 / m.len() as f64;
            for v in m {
                *v += val;
            }
        }
    }

    matrix
}

pub fn print_matrix(matrix: &Vec<Vec<f64>>, row : &Vec<char>, col : &Vec<char>) {
    for c in col {
        print!("\t{c}");
    }
    println!();
    for i in 0..row.len() {
        print!("{}\t", row[i]);
        for j in 0..col.len() {
            print!("{:}\t", matrix[i][j]);
        }
        println!();
    }
}

pub fn run(content: Vec<String>) {
    let text = &content[0];
    let alphabet: Vec<char> = content[2].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();
    let path = &content[4];
    let states : Vec<char> = content[6].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect(); 

    let transition_matrix = transition(path, &states);
    let emission_matrix = emission(text, path, &alphabet, &states);
    print_matrix(&transition_matrix, &states, &states);
    println!("--------");
    print_matrix(&emission_matrix, &states, &alphabet);
}