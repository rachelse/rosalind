pub mod solution;
pub mod kmer;
pub mod nucleotide;
pub mod mathematics;

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn print_answer<T: std::fmt::Display>(ans :Vec<T>) {
    for a in ans {
        print!("{a} ");
    }
    println!();
}

// TODO: Make function to write answer
// pub fn write_answer

pub mod FileReader {
    use crate::*;

    pub struct Path(String);

    impl Path {
        pub fn get(args: &[String]) ->Result<Self, &'static str> {
            if args.len() != 2 {
                return Err("Only 1 argument is allowed");
            }

            Ok(Path(args[1].to_string()))
        }

        pub fn read(path: &Self) -> Result<Vec<String>, &str> {
            let data = File::open(&path.0).unwrap_or_else(|err| panic!("Error opening file {}",err));
    
            let reader = BufReader::new(data);
            let lines: Vec<String> = reader.lines().collect::<Result<_,_>>().unwrap();
    
            Ok(lines)
        }
    }

}
