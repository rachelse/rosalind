/* Implement Baum-Welch Learning */
use super::{C,H, J,A};
use super::D::Viterbi as forward;

fn Estep(x: &String, alphabet: &Vec<char>, states: &Vec<char>, transition: &Vec<Vec<f64>>, emission: &Vec<Vec<f64>>)-> String{
    let (score, tracker) = C::Viterbi(x, &alphabet, &states, &transition, &emission);
    let path = C::decode(score, &states, &tracker);
    path
}

fn Mstep(transition: &Vec<Vec<f64>>, emission: &Vec<Vec<f64>>, x: &String, path: &String, states: &Vec<char>, alphabet: &Vec<char>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut e_responsibility = vec![vec![0.0;states.len()];x.len()];
    let mut t_responsibility = vec![vec![0.0;states.len()*states.len()];x.len()-1];

    let forward_prob = forward(x, &alphabet, &states, &transition, &emission);
    let pr = forward_prob.last().unwrap().iter().sum::<f64>();    
    let backward_prob = J::backward(x, &alphabet, &states, &transition, &emission);

    /* calculate e_responsibility */
    for i in 0..x.len() {
        for (j, (f, b)) in forward_prob[i].iter().zip(backward_prob[i].iter()).enumerate() {
            e_responsibility[i][j] =f*b/pr ;
        }
    }

    /* calculate t_responsibility */
    for i in 0..x.len()-1 {
        let mut j = 0;
        for l in 0..states.len() {
            for k in 0..states.len() {
                let xi1 = A::state_to_idx(alphabet, x.chars().nth(i+1).unwrap());
                let weight = transition[l][k] * emission[k][xi1]; 
                t_responsibility[i][j] = forward_prob[i][l] * backward_prob[i+1][k] * weight / pr;
                j+=1;
            }
        }
    }

    /* modify emission matrix */ 
    let mut emission_updated = vec![vec![0.0;alphabet.len()];states.len()];
    for s in 0..states.len() {
        for a in 0..x.len() {
            let m = A::state_to_idx(alphabet,x.chars().nth(a).unwrap());
            emission_updated[s][m] += e_responsibility[a][s];
        }
    }

    for (i, vec) in emission_updated.iter_mut().enumerate() {
        let total = vec.iter().sum::<f64>();
        for v in vec {
            *v /= total;
        }
    }

    /* modify transmission matrix */
    let mut transition_updated = vec![vec![0.0;states.len()];states.len()];
    let mut row_total = vec![0.0;states.len().pow(2)];
    for vec in t_responsibility {
        for (i, v) in vec.iter().enumerate() {
            row_total[i] += v;
        }
    }
    let mut idx = 0;
    for i in 0..states.len() {
        let mut total = row_total[i*states.len()..(i+1)*states.len()].iter().sum::<f64>();
        for j in 0..states.len() {
            transition_updated[i][j] = row_total[idx] / total;
            idx+=1
        }
    }

    (transition_updated, emission_updated)
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
        let path = Estep(x, &alphabet, &states, &transition, &emission);
        (transition, emission) = Mstep(&transition, &emission, x, &path, &states, &alphabet);
    }

    H::print_matrix(&transition, &states, &states);
    println!("--------");
    H::print_matrix(&emission, &states, &alphabet);
}