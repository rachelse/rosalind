/* Reconstruct a String from its Burrows-Wheeler Transform */
use std::collections::HashMap;

pub fn numberFirst(last: &Vec<String>) -> Vec<String> {
    let mut first : Vec<String> = last.clone();
    first.sort_by(|x, y| 
        if x[..1] != y[..1] {
            x.cmp(y)
        } else {
         x[1..].parse::<usize>().unwrap().cmp(&y[1..].parse::<usize>().unwrap())});
    first
}

pub fn numberLast(last: &Vec<char>) -> Vec<String> {
    let mut counter: HashMap<char,usize> = HashMap::new();
    let mut last_str : Vec<String> = Vec::new();
    for c in last {
        let ch = counter.entry(*c).or_insert(0);
        *ch+=1;
        last_str.push(format!("{c}{ch}"));
    } 
    last_str
}

pub fn reconstruct(last: &Vec<char>)-> String {
    let mut last_str = numberLast(&last);
    let mut first = numberFirst(&last_str);

    let mut text = String::new();
    let mut last_chr = "$1".to_string();

    while text.len() < last.len() {
        let pos = last_str.iter().position(|x| x == &last_chr).unwrap();
        let next = first.get(pos).unwrap();

        last_chr = next.to_owned();
        text.push(next.chars().nth(0).unwrap());
    }

    text
}

pub fn run(content:Vec<String>) {
    let last = &content[0];
    let last : Vec<char> = last.chars().collect();

    let mut text = reconstruct(&last);
    println!("{text}");
}