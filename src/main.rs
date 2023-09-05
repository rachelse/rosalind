use std::env;
use rosalind::{FileReader, 
                solution::*,
            * };

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = FileReader::Path::get(&args).unwrap();
    let mut content = FileReader::Path::read(&path).unwrap();


    // BA5::A::run(&mut content);
}