pub fn solve(pattern: &String) {
    let n = pattern.len();
    let mut i: usize = 0;
    let mut number = 0;
    while i < n {
        let nn = &pattern[i..i+1];
        let nth = 4_i128.pow((n-1-i).try_into().unwrap());
        match nn {
            "A" => {number += 0*nth},
            "C" => {number += 1*nth},
            "G" => {number += 2*nth},
            "T" => {number += 3*nth},
            _ => ()
        }
        i += 1;
    }
    println!("{number}");
}

pub fn run(content : &Vec<String>) {
    let pattern = content.get(0).unwrap();
    solve(pattern)
}