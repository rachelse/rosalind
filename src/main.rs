use std::env;
use rosalind::{FileReader, 
                solution::*,
            * };

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = FileReader::Path::get(&args).unwrap();
    let content = FileReader::Path::read(&path).unwrap();

    // BA4::D::run(content);
    // stronghold::MOTZ::run(content);
    algorithm::DDEG::run(content);
}