/* Generate the Last-to-First Mapping of a String */
use crate::solution::BA9::J;

pub fn LastToFirst(last: &Vec<char>) -> Vec<usize> {
    let mut last_str = J::numberLast(&last);
    let mut first = J::numberFirst(&last_str);
    let mut l_to_f = Vec::new();
    for l in last_str {
        l_to_f.push(first.iter().position(|x| x==&l).unwrap());
    }
    l_to_f
}

pub fn run(content: Vec<String>) {
    let transform = &content[0];
    let transform : Vec<char> = transform.chars().collect();
    let i = content[1].parse::<usize>().unwrap();

    let last_to_first = LastToFirst(&transform);

    println!("{}", last_to_first[i]);
}