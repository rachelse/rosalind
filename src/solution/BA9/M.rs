/* Implement BetterBWMatching */
use std::collections::HashMap;

pub fn symbol_to_idx(symbol : char ) -> Option<usize> {
    match symbol {
        '&' => {return Some(0)},
        'A' => {return Some(1)},
        'C' => {return Some(2)},
        'G' => {return Some(3)},
        'T' => {return Some(4)},
        _ => {return None}
    }
}

pub fn firstOccurence(firstcol: &Vec<char>) -> HashMap<char, usize> {
    let mut first_occurence = HashMap::new();
    for (i, c) in firstcol.iter().enumerate() {
        first_occurence.entry(*c).or_insert(i);
    }
    first_occurence
}

pub fn countSymbol(lastcol : &Vec<char>) -> Vec<Vec<usize>> {
    // symbol: char, i:usize, 
    let mut cnt_matrix = vec![]; // $, A, C, G, T
    let mut ith_matrix = vec![0;5];
    cnt_matrix.push(ith_matrix.clone());

    for i in 0..lastcol.len() {
        match lastcol[i] {
            '$' => {ith_matrix[0] +=1;},
            'A' => {ith_matrix[1] +=1;},
            'C' => {ith_matrix[2] +=1;},
            'G' => {ith_matrix[3] +=1;},
            'T' => {ith_matrix[4] +=1;}
            _ => ()
        }
        cnt_matrix.push(ith_matrix.clone());
    }
    cnt_matrix
}

fn BetterBWMatching(first_occurence : &HashMap<char,usize>, lastcol : &Vec<char>, pattern: &str, count_matrix: &Vec<Vec<usize>>) -> usize {
    let mut top = 0;
    let mut bottom = lastcol.len()-1;
    let mut pattern = pattern.to_string();
    while top <= bottom {
        if pattern.len() != 0 {
            let symbol = pattern.remove(pattern.len()-1);
            let mut exist = false;
            for i in top..bottom+1 {
                if lastcol[i] == symbol {
                    exist = true;
                    break
                }
            }
            if exist {
                top = first_occurence[&symbol] + count_matrix[top][symbol_to_idx(symbol).unwrap()];
                bottom = first_occurence[&symbol] + count_matrix[bottom+1][symbol_to_idx(symbol).unwrap()]-1;
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

    let first_occurence = firstOccurence(&first_col);
    let count_matrix = countSymbol(&last_col);
    for pattern in patterns {
        let matches = BetterBWMatching(&first_occurence, &last_col,  pattern, &count_matrix);
        print!("{matches} ");
    }
    println!();
}