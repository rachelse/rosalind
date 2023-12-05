/* Solve the Soft Decoding Problem */
use super::{A,C};
use super::D::Viterbi as forward;

pub fn backward(x: &String, hidden: &Vec<char> , states: &Vec<char>, transition: &Vec<Vec<f64>>, emission: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut backward: Vec<Vec<f64>> = Vec::new();
    let x_rev : String = x.chars().rev().collect();
    for i in 0..x_rev.len() {
        let mut weight = vec![0.0;states.len()];

        if i != 0 {
            let last = &backward.last().unwrap();

            for ith_1 in 0..weight.len() {
                for (ith, l) in last.iter().enumerate() {
                    let h_idx = A::state_to_idx(&hidden, x_rev.chars().nth(i-1).unwrap());
                    weight[ith_1] += l * transition[ith_1][ith] * emission[ith][h_idx]; /* backward * transition * emission */
                }
            }
        } else {
            for w in weight.iter_mut() {
                *w += 1.0;
            }
        }
        backward.push(weight);
    }

    backward.reverse();
    backward
}

pub fn run(content: Vec<String>) {
    let x = &content[0];
    let alphabet : Vec<char> = content[2].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();
    let states : Vec<char> = content[4].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();

    let mut transition : Vec<Vec<f64>> = Vec::new();
    for i in 7..7+states.len() {
        transition.push(C::_parse_line(&content[i]));
    }

    let mut emission : Vec<Vec<f64>> = Vec::new();
    for i in 9+states.len()..content.len() {
        emission.push(C::_parse_line(&content[i]));
    }

    let forward_prob = forward(x, &alphabet, &states, &transition, &emission);
    let pr = forward_prob.last().unwrap().iter().sum::<f64>();    
    let backward_prob = backward(x, &alphabet, &states, &transition, &emission);

    /* print answer */
    for state in states {
        print!("{state}\t");
    }
    println!();
    for i in 0..forward_prob.len() {
        for (j, (f, b)) in forward_prob[i].iter().zip(backward_prob[i].iter()).enumerate() {
            print!("{:.4}\t", f*b/pr);
        }
        println!();
    }
}