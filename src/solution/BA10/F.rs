/* Construct a Profile HMM with Pseudocounts */
use super::E;

fn addPseudocount(matrix: &mut Vec<Vec<f64>>, row : (usize, usize), col: (usize,usize), pseudocount: f64) {
    for r in row.0..=row.1 {
        for c in col.0..=col.1 {
            matrix[r][c] += pseudocount;
        }
    }
}

pub fn Pseudocount(pseudocount: f64, transition: &mut Vec<Vec<f64>>, emission:&mut Vec<Vec<f64>>, num_seeds: usize, num_alphabets: usize) {
    let mut row = (0,0);
    let mut col = (0,0);
    for i in 0..=num_seeds {
        if i == 0 {
            row = (0,1);
            col = (i*3+1,i*3+3);
        } else if i == num_seeds {
            row = (i*3-1,i*3+1);
            col = (i*3+1,i*3+2);
        } else {
            row = (i*3-1,i*3+1);
            col = (i*3+1,i*3+3);
        }
        addPseudocount(transition, row, col, pseudocount);
    
        if i == num_seeds {
            addPseudocount(emission, (i*3+1,i*3+1), (0,num_alphabets-1), pseudocount);
        } else {            
            addPseudocount(emission, (i*3+1,i*3+2), (0,num_alphabets-1), pseudocount);
        }
    }
}

pub fn run(content: Vec<String>) {
    let first_line : Vec<f64> = content[0].split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();
    let (threshold, pseudocount) = (first_line[0], first_line[1]);
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

    let mut seed = E::build_seed(&alignment, threshold);

    let (mut transition, mut emission) = E::Profile(&alignment,&mut seed, &alphabet);


    /* divide matrix with sum of each vector */
    E::format_matrix(&mut transition);
    E::format_matrix(&mut emission);

    /* add pseudocounts */
    Pseudocount(pseudocount,&mut transition, &mut emission, seed.len(), alphabet.len());

    /* divide matrix with sum of each vector */
    E::format_matrix(&mut transition);
    E::format_matrix(&mut emission); 

    let mut seed_vec = vec![String::from("S"), String::from("I0")] ;
    for i in 0..seed.len() {
        seed_vec.push(format!("M{}",i+1));
        seed_vec.push(format!("D{}",i+1));
        seed_vec.push(format!("I{}",i+1));
    }
    seed_vec.push(String::from("E"));

    E::print_matrix(&transition, &seed_vec, &seed_vec);
    println!("--------");
    E::print_matrix(&emission, &seed_vec, &alphabet);
}