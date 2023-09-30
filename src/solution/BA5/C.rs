/* Find a Longest Common Subsequence of Two Strings */

use std::cmp;

#[derive(Clone)]
enum direction {
    Down,
    Right,
    Diagonal,
    None,
}

pub fn find_lcs(s1: &String, s2: &String) -> String {
    // Find matches
    let l1 = s1.len();
    let l2 = s2.len();
    let mut matches = vec![vec![0;l1+1];l2+1];
    let mut track =vec![vec![direction::None;l1+1];l2+1];
    let (mut max, mut f1, mut f2) = (0,0,0); // variables for back track

    for i1 in 1..=l1 {
        for i2 in 1..=l2 {
            let mut max_val = cmp::max(matches[i2-1][i1], matches[i2][i1-1]);
            if s1[i1-1..i1] == s2[i2-1..i2] {
                let matched =matches[i2-1][i1-1] + 1; 
                if matched >= max_val {
                    max_val = matched;
                    if max_val > max {
                        (max,f1,f2) = (max_val,i1,i2);
                    }
                }
            }
            matches[i2][i1] = max_val;

            match max_val {
                x if max_val == matches[i2][i1-1] => {track[i2][i1] = direction::Right},
                x if max_val == matches[i2-1][i1] => {track[i2][i1] = direction::Down},
                x if max_val == matches[i2-1][i1-1]+1 => {track[i2][i1]= direction::Diagonal},
                _ => ()
            }
        }
    }

    // Track lcs
    let mut lcs_rev= String::new();

    while f1>=1 && f2>=1 {
        match track[f2][f1] {
            direction::Down => {f2-=1;},
            direction::Right => {f1-=1;},
            direction::Diagonal => {lcs_rev.push_str(&s1[f1-1..f1]);
                                    f1-=1; f2-=1;},
            _ => ()
        }
    }
    lcs_rev.chars().rev().collect()
}

pub fn run(content : &Vec<String>) {
    let lcs = find_lcs(&content[0], &content[1]);
    println!("{lcs}");
}