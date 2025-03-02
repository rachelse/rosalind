use std::env;
use rosalind::{FileReader, 
                solution::*,
            * };

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = FileReader::Path::get(&args).unwrap();
    let content = FileReader::Path::read(&path).unwrap();

    BA9::P::run(content);
    // BA4::B::run(content);
    // stronghold::MOTZ::run(content);
}