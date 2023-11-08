/* Implement BWMatching */
use crate::solution::BA9::K;

fn BWMatching(firstcol : &Vec<char>, lastcol : &Vec<char>, pattern: &str, last_to_first: &Vec<usize>) -> usize {
    let mut top = 0;
    let mut bottom = lastcol.len()-1;
    let mut pattern = pattern.to_string();
    while top <= bottom {
        if pattern.len() != 0 {
            let symbol = pattern.remove(pattern.len()-1);
            let mut exist = false;
            let mut positions = Vec::new();
            for i in top..bottom+1 {
                if lastcol[i] == symbol {
                    exist = true;
                    positions.push(i);
                }
            }
            if exist {
                top = last_to_first[*positions.first().unwrap()];
                bottom = last_to_first[*positions.last().unwrap()]
            } else {
                return 0
            }
        } else {
            return bottom - top + 1
        }
    }
    return 0
}

pub fn run(content: Vec<String>) {
    let text = &content[0];
    let patterns: Vec<&str> = content[1].split_whitespace().collect();

    let last_col : Vec<char> = text.chars().collect();
    let mut first_col = last_col.clone();
    first_col.sort();
    let l_to_f = K::LastToFirst(&last_col);

    for pattern in patterns {
        let matches = BWMatching(&first_col, &last_col,  pattern, &l_to_f);
        print!("{matches} ");
    }
    println!();
}