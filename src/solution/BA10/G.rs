/* Perform a Multiple Sequence Alignment with a Profile HMM  */

use core::fmt;
use crate::utils::float::*;
use super::{E,F, A};

fn print_transition(matrix: &Vec<Vec<f64>>, rownames: &Vec<State>) {
    for s in rownames {
        print!("\t{s}");
    }
    println!();

    for i in 0..matrix.len() {
        print!("{}\t", rownames[i]);
        for v in &matrix[i] {
            print!("{:.3}\t",v);
        }
        println!();
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum State {
    Insertion(usize),
    Deletion(usize),
    Match(usize),
    Start,
    End,
    None
}

impl State {
    fn get_seed_num(&self) -> Option<usize> {
        match self {
            State::Deletion(x) | State::Insertion(x) | State::Match(x) => {return Some(*x)},
            _ => {return None}
        }
    }

    fn state_to_idx(&self, rownames: &Vec<State>) -> usize {
        rownames.iter().position(|x| x==self).unwrap()
    }

    fn idx_to_state(idx:usize, rownames: &Vec<State>) -> Self {
        rownames[idx]
    }

    fn get_all_prev(&self) -> Vec<State> {
        let seed = self.get_seed_num();
        if let Some(x) = seed {
            if let State::Insertion(n) = self {
                if *n == 0 {
                    return vec![self.clone()] 
                } else {
                    return vec![State::Match(*n), State::Deletion(*n), State::Insertion(*n)]
                }
            } else {
                if x == 1 {
                    if let State::Match(n) = self {
                        return vec![State::Insertion(n-1)]
                    } else {
                        return vec![State::Insertion(x-1)]
                    }
                } else {
                    return vec![State::Match(x-1), State::Deletion(x-1), State::Insertion(x-1)]
                }
            }
        } else {
            return Vec::new()
        }

    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f:&mut fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Deletion(n) => {write!(f, "D{n}")}
            State::Match(n) => {write!(f, "M{n}")} 
            State::Insertion(n) => {write!(f, "I{n}")}
            State::Start => {write!(f, "S")}
            State::End => {write!(f, "E")}
            _ => {write!(f,"wrong format")}
        }
    }
}

pub fn track(score: &Vec<Vec<f64>>, tracker:&Vec<Vec<State>>, rownames: &Vec<State>) -> Vec<State>{
    let mut idx = get_max_f64(score.last().unwrap());
    let mut col = score.len();
    

    let mut tracked = vec![];

    let mut state = State::idx_to_state(idx, rownames);
    tracked.push(state.clone());
    
    while state != State::Start {
        match State::idx_to_state(idx, rownames) {
            State::Deletion(_) => (),
            _ => {col-=1;}
        }

        idx = state.state_to_idx(rownames);
        state = tracker[col][idx];
        tracked.push(state);
    }

    tracked.reverse();
    
    tracked
}

pub fn MultipleSequenceAlignment(text : &String, transition: &Vec<Vec<f64>>, emission: &Vec<Vec<f64>>, num_seeds: usize, transition_names: &Vec<State>, alphabet:&Vec<char>) -> (Vec<Vec<f64>>, Vec<Vec<State>>) {
    let mut viterbi_names = transition_names.clone();
    viterbi_names.remove(0);
    viterbi_names.pop();

    /* initialize matrices */
    let mut viterbi_graph : Vec<Vec<f64>> = vec![vec![0.0;transition_names.len()-2];text.len()+1];
    let mut viterbi_tracker : Vec<Vec<State>> = vec![vec![State::None;transition_names.len()-2];text.len()+1];

    /* fill matrices */
    for c in 0..viterbi_graph.len() {
        let row = viterbi_graph[c].len();
        /* Start to Deletions */
        if c == 0 {
            let mut prev_state = State::Start;
            let mut trans_prev = 0;
            let mut prob = 1.0;
            for r in 1..=num_seeds {
                let cur_state = State::Deletion(r);
                let trans_cur = cur_state.state_to_idx(transition_names);
                let viterbi_cur = cur_state.state_to_idx(&viterbi_names);
                prob *= transition[trans_prev][trans_cur];
                viterbi_graph[c][viterbi_cur] = prob;
                viterbi_tracker[c][viterbi_cur] = prev_state;
                trans_prev = trans_cur;
                prev_state = cur_state;
            }
            continue
        }

        /* each alphabet of text */
        let t = text.chars().nth(c-1).unwrap();
        for r in 0..row {
            let mut max_prob = 0.0;
            let mut max_state = State::None;

            let current_state = State::idx_to_state(r, &viterbi_names);
            let trans_cur = current_state.state_to_idx(&transition_names);
            let mut prev_states = current_state.get_all_prev();
            if c == 1 {
                match current_state {
                    State::Insertion(0) => {prev_states.clear(); prev_states.push(State::Start);}
                    State::Match(1) => {prev_states.clear(); prev_states.push(State::Start);}
                    State::Insertion(n) => {prev_states.clear();prev_states.push(State::Deletion(n))}
                    State::Match(n) => {prev_states.clear();prev_states.push(State::Deletion(n-1))}
                    State::Deletion(_) => (),
                    _ => ()
                }
            }

            for prev_state in prev_states {
                let trans_prev = prev_state.state_to_idx(&transition_names);
                let t_idx = A::state_to_idx(&alphabet, t);
                let mut prob = transition[trans_prev][trans_cur];

                if prev_state == State::Start { /* Start to Match1 */
                    prob *= emission[trans_cur][t_idx];
                } else {
                    let viterbi_prev = prev_state.state_to_idx(&viterbi_names);
                    if let State::Deletion(_) = current_state {
                        prob *= viterbi_graph[c][viterbi_prev];
                    } else {
                        prob *= viterbi_graph[c-1][viterbi_prev];
                        prob *= emission[trans_cur][t_idx];
                    }
                }

                if prob > max_prob {
                    max_prob = prob;
                    max_state = prev_state;
                }
            } 

            viterbi_graph[c][r] = max_prob;
            viterbi_tracker[c][r] = max_state;
        }
    }

    (viterbi_graph, viterbi_tracker)
}

pub fn run(content: Vec<String>) {
    let text = &content[0];
    let condition : Vec<f64> = content[2].split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();
    let (threshold, pseudocount) = (condition[0], condition[1]);
    let alphabet: Vec<char> = content[4].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();

    let mut alignment : Vec<Vec<char>> = Vec::new();

    for i in 6..content.len() {
        if i == 6 {
            for _ in 0..content[i].len() {alignment.push(Vec::new())}
        }
        let seq_chars : Vec<char> = content[i].chars().collect();
        for (i, c) in seq_chars.iter().enumerate() {
            alignment[i].push(*c);
        }
    }

    let mut seed = E::build_seed(&alignment, threshold);

    let (mut transition, mut emission) = E::Profile(&alignment,&mut seed, &alphabet);


    /* divide matrix with sum of each vector */
    E::format_matrix(&mut transition);
    E::format_matrix(&mut emission);

    /* add pseudocounts */
    F::Pseudocount(pseudocount,&mut transition, &mut emission, seed.len(), alphabet.len());

    /* divide matrix with sum of each vector */
    E::format_matrix(&mut transition);
    E::format_matrix(&mut emission); 

    let mut seed_vec = vec![State::Start, State::Insertion(0)] ;
    for i in 0..seed.len() {
        seed_vec.push(State::Match(i+1));
        seed_vec.push(State::Deletion(i+1));
        seed_vec.push(State::Insertion(i+1));
    }
    seed_vec.push(State::End);

    let (score, tracker) = MultipleSequenceAlignment(text,&transition, &emission, seed.len(), &seed_vec, &alphabet);

    let mut rownames = seed_vec.clone();
    rownames.pop();
    rownames.remove(0);
    let tracked = track(&score, &tracker, &rownames);

    for i in 1..tracked.len() {
        print!("{} ",tracked[i])
    }
    println!()

}