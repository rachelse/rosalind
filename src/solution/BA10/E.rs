/* Construct a Profile HMM */
use super::A;

fn _get_state_idx(idx: usize, state: &str) -> usize{
    match state {
        "match" => {return 3*(idx+1)-1},
        "deletion" => {return 3*(idx+1)},
        "insertion" => {return 3*(idx+1)+1},
        _ => {return usize::MAX}
    }
}

pub fn build_seed(alignment: &Vec<Vec<char>>, threshold : f64) -> Vec<usize> {
    let mut seed: Vec<usize> = Vec::new();
    for (i, aligned) in alignment.iter().enumerate() {
        let gap_cnt = aligned.iter().filter(|&x| *x== '-').count();
        if (gap_cnt as f64/ aligned.len() as f64) < threshold {seed.push(i)}
    }
    seed 
}

fn count(alphabet: &Vec<char>, alignment: &Vec<char>) -> Vec<f64> {
    let mut counter = vec![0.0;alphabet.len()+1];
    for c in alignment {
        if *c == '-' {
            counter[alphabet.len()] += 1.0;
        } else {
            let idx = A::state_to_idx(alphabet, *c);
            counter[idx] += 1.0;
        }
    }
    counter
}

pub fn format_matrix(matrix: &mut Vec<Vec<f64>>) {
    for mut line in matrix.iter_mut() {
        let total = line.iter().sum::<f64>();
        if total != 0.0 {
            for v in line {
                *v = *v/total;
            }
        }
    }
}

fn fill_transition(transition: &mut Vec<Vec<f64>>, row: usize, col: usize, val:Option<f64>) {
    if val==None {
        transition[row][col] += 1.0;
    } else {
        transition[row][col] += val.unwrap();
    }

}

fn seed_to_seed(transition: &mut Vec<Vec<f64>>,  alignment: &Vec<Vec<char>>, prev_idx: usize, seed_val: (usize,usize)) {
    for (before, after) in alignment[seed_val.0].iter().zip(&alignment[seed_val.1]) {
        match (*before,*after) {
            ('-', '-') => {fill_transition(transition, _get_state_idx(prev_idx, "deletion"), _get_state_idx(prev_idx +1, "deletion"), None)},
            ('-', _) => {fill_transition(transition, _get_state_idx(prev_idx, "deletion"), _get_state_idx(prev_idx +1, "match"), None)},
            (_, '-') => {fill_transition(transition, _get_state_idx(prev_idx, "match"), _get_state_idx(prev_idx +1, "deletion"), None)},
            _ => {fill_transition(transition, _get_state_idx(prev_idx, "match"), _get_state_idx(prev_idx +1, "match"), None)}
        }
    }
}

fn seed_insert(transition: &mut Vec<Vec<f64>>, alignment: &Vec<Vec<char>>, prev_idx: usize, seed_val: (usize,usize)) {
    let n_seq = alignment[0].len();
    for i in 0..n_seq {
        let mut seq = Vec::new();
        let mut insertion_cnts = 0;
        for s in seed_val.0..=seed_val.1 {
            seq.push(alignment[s][i]);
            if s>seed_val.0 && s<seed_val.1  && alignment[s][i]!='-'{
                insertion_cnts+=1;
            }
        }
        if *seq.first().unwrap() != '-' { /* Match to  */
            if insertion_cnts == 0 { /* No insertion */
                if *seq.last().unwrap() == '-' {
                    fill_transition(transition, _get_state_idx(prev_idx, "match"), _get_state_idx(prev_idx+1, "deletion"), None);
                } else {
                    fill_transition(transition, _get_state_idx(prev_idx, "match"), _get_state_idx(prev_idx+1, "match"), None);
                }
            } else {
                fill_transition(transition, _get_state_idx(prev_idx, "match"), _get_state_idx(prev_idx, "insertion"), None); 
                insert_seed(transition, prev_idx, insertion_cnts, *seq.last().unwrap());
            }
        } else { /* deletion to */
            if insertion_cnts == 0 {
                if *seq.last().unwrap() == '-' {
                    fill_transition(transition, _get_state_idx(prev_idx, "deletion"), _get_state_idx(prev_idx+1, "deletion"), None);
                } else {
                    fill_transition(transition, _get_state_idx(prev_idx, "deletion"), _get_state_idx(prev_idx+1, "match"), None);
                }
            } else {
                fill_transition(transition, _get_state_idx(prev_idx, "deletion"), _get_state_idx(prev_idx, "insertion"), None); 
                insert_seed(transition, prev_idx, insertion_cnts, *seq.last().unwrap());
            } 
        }
    }
}

fn insert_seed(transition: &mut Vec<Vec<f64>>, prev_idx: usize, insertion_cnts:usize, last : char) {
    if last == '-' {
        fill_transition(transition, _get_state_idx(prev_idx, "insertion"), _get_state_idx(prev_idx+1, "deletion"), None);
    } else {
        fill_transition(transition, _get_state_idx(prev_idx, "insertion"), _get_state_idx(prev_idx+1, "match"), None);
    }
    fill_transition(transition, _get_state_idx(prev_idx, "insertion"), _get_state_idx(prev_idx, "insertion"), Some(insertion_cnts as f64 - 1.0)); 
}

fn fill_emission(emission: &mut Vec<Vec<f64>>, idx: usize, counter:&Vec<f64>, alphabet: &Vec<char>) {
    for i in 0..alphabet.len() {
        emission[idx][i] += counter[i];
    }
}

pub fn print_matrix<T: std::fmt::Display> (matrix: &Vec<Vec<f64>>, row: &Vec<String>, col: &Vec<T>) {
    for i in col {
        print!("\t{i}");
    }
    println!();
    for (i, r) in row.iter().enumerate() {
        let mut line = String::from(r);
        for f in &matrix[i] {
            line.push('\t');
            line.push_str(format!("{:.3}", f).as_str());
        }
        println!("{line}");
    }
}

pub fn Profile(alignment: &Vec<Vec<char>>, seed:&mut Vec<usize>, alphabet: &Vec<char>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let len_seq = alignment.len();
    let n_seq = alignment[0].len();
    let n_seed = seed.len();

    let mut transition = vec![vec![0.0;3*(n_seed+1)];3*(n_seed+1)];
    let mut emission = vec![vec![0.0;alphabet.len()];3*(n_seed+1)];

    let (mut prev_seed, mut prev_idx) = (*seed.first().unwrap(),0);

    if prev_seed == 0 {
        let counter = count(&alphabet, &alignment[0]);
        let d = counter[alphabet.len()];
        let m = counter.iter().sum::<f64>() - d;
        fill_transition(&mut transition, 0, _get_state_idx(prev_idx, "match"), Some(m));
        fill_transition(&mut transition, 0, _get_state_idx(prev_idx, "deletion"), Some(d));
        fill_emission(&mut emission, _get_state_idx(prev_idx, "match"), &counter, alphabet);
    } else {
        /* Until the first seed */
        for i in 0..n_seq {
            let mut seq = Vec::new();
            let mut insertion_cnts = 0;
            for s in 0..=prev_seed {
                seq.push(alignment[s][i]);
                if s<prev_seed  && alignment[s][i]!='-'{
                    insertion_cnts+=1;
                }
            }
            if insertion_cnts == 0 {
                if *seq.last().unwrap() != '-' {
                    fill_transition(&mut transition, 0, _get_state_idx(prev_idx, "match"), None)
                } else {
                    fill_transition(&mut transition, 0, _get_state_idx(prev_idx, "deletion"), None) 
                }
                let counter = count(alphabet, &vec![*seq.last().unwrap()]) ;
                fill_emission(&mut emission, _get_state_idx(prev_idx, "match"), &counter, alphabet)
            } else {
                fill_transition(&mut transition, 0, 1, None);
                if *seq.last().unwrap() != '-' {
                    fill_transition(&mut transition, 1, _get_state_idx(prev_idx, "match"), None);
                } else {
                    fill_transition(&mut transition, 1, _get_state_idx(prev_idx, "deletion"), None);
                }
                fill_transition(&mut transition, 1, 1, Some(insertion_cnts as f64 - 1.0));  

                let seed = vec![seq.remove(seq.len()-1)];
                let seed_counter = count(alphabet, &seed);
                fill_emission(&mut emission,_get_state_idx(prev_idx, "match"), &seed_counter, alphabet);
                let insertion_counter = count(alphabet, &seq);
                fill_emission(&mut emission,1, &insertion_counter, alphabet);
            }
        }
    }

    /* 2nd seed to til before last seed*/
    while seed.len() > prev_idx+1 {
        let current_seed = seed[prev_idx+1];
        let seed_counter = count(alphabet, &alignment[current_seed]);
        fill_emission(&mut emission, _get_state_idx(prev_idx+1, "match"), &seed_counter, alphabet);

        if current_seed - prev_seed == 1 { /* if insertion not exist */
            seed_to_seed(&mut transition, &alignment, prev_idx, (prev_seed,current_seed));
            prev_seed = current_seed;
            prev_idx+=1
        } else { /* if insertion is in the middle */
            seed_insert(&mut transition, alignment, prev_idx, (prev_seed,current_seed));

            let mut insertion_chars = Vec::new();
            for i in prev_seed+1..current_seed {
                insertion_chars.extend(alignment[i].clone());
            }
            let insertion_counter =count(alphabet, &insertion_chars);
            fill_emission(&mut emission, _get_state_idx(prev_idx, "insertion"), &insertion_counter, alphabet);

            prev_seed = current_seed;
            prev_idx+=1;
        }
    }

    /* to END */
    let end_idx = 3*(n_seed+1)-1;
    if prev_seed == len_seq-1 { /* if last column was seed */
        let counter = count(&alphabet, &alignment[len_seq-1]);
        let d = counter[alphabet.len()];
        let m = counter.iter().sum::<f64>() - d;
        fill_transition(&mut transition, _get_state_idx(prev_idx, "match"), end_idx, Some(m));
        fill_transition(&mut transition, _get_state_idx(prev_idx, "deletion"), end_idx, Some(d));
    } else { 
        for i in 0..n_seq {
            let mut seq = Vec::new();
            let mut insertion_cnts = 0;
            for s in prev_seed..len_seq {
                seq.push(alignment[s][i]);
                if s>prev_seed  && alignment[s][i]!='-'{
                    insertion_cnts+=1;
                }
            }
            
            if insertion_cnts == 0 { /* if no insertion */
                if *seq.first().unwrap() != '-' {
                    fill_transition(&mut transition, _get_state_idx(prev_idx, "match"), end_idx, None);
                } else {
                    fill_transition(&mut transition, _get_state_idx(prev_idx, "deletion"), end_idx, None);
                }
            } else {
                fill_transition(&mut transition, _get_state_idx(prev_idx, "insertion"), end_idx, None);
                if *seq.first().unwrap() != '-' {
                    fill_transition(&mut transition, _get_state_idx(prev_idx, "match"), end_idx-1,None);
                    
                    seq.remove(0);
                    let insertion_counter = count(alphabet, &seq);
                    fill_emission(&mut emission,_get_state_idx(prev_idx, "insertion"), &insertion_counter, alphabet);
                } else {
                    fill_transition(&mut transition, _get_state_idx(prev_idx, "deletion"), end_idx-1, None);
                }
                fill_transition(&mut transition, _get_state_idx(prev_idx, "insertion"), _get_state_idx(prev_idx, "insertion"), Some(insertion_cnts as f64 - 1.0));  
            }
        } 
    }

    (transition, emission)
}

pub fn run(content: Vec<String>) {
    let threshold = content[0].parse::<f64>().unwrap();
    let alphabet: Vec<char> = content[2].split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();

    let mut alignment : Vec<Vec<char>> = Vec::new();

    for i in 4..content.len() {
        if i == 4 {
            for _ in 0..content[i].len() {alignment.push(Vec::new())}
        }
        let seq_chars : Vec<char> = content[i].chars().collect();
        for (i, c) in seq_chars.iter().enumerate() {
            alignment[i].push(*c);
        }
    }

    let mut seed = build_seed(&alignment, threshold);

    let (mut transition, mut emission) = Profile(&alignment,&mut seed, &alphabet);
    let mut seed_vec = vec![String::from("S"), String::from("I0")] ;
    for i in 0..seed.len() {
        seed_vec.push(format!("M{}",i+1));
        seed_vec.push(format!("D{}",i+1));
        seed_vec.push(format!("I{}",i+1));
    }
    seed_vec.push(String::from("E"));

    /* divide matrix with sum of each vector */
    format_matrix(&mut transition);
    format_matrix(&mut emission);

    print_matrix(&transition, &seed_vec, &seed_vec);
    println!("--------");
    print_matrix(&emission, &seed_vec, &alphabet);
}