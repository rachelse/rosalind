use std::{
        error::Error};

#[derive(Debug)]
pub struct ba1hParser {
    pub pattern:String,
    pub text:String,
    pub d:u8,
}

impl ba1hParser {

    pub fn build(lines : &[String] ) -> Result<ba1hParser, &'static str> {
       
        if lines.len() !=3 {
            return Err("Wrong data format");
        } else {
            Ok(ba1hParser { 
                pattern: lines[0].clone(), 
                text: lines[1].clone(),
                d: lines[2].clone().parse::<u8>().unwrap() })
        }
    }

}

pub fn solve(data: &ba1hParser) -> Result<Vec<&str>,Box<dyn Error>> {
    let pat_len = data.pattern.len();
    let seq_len = data.text.len();


    let mut dna_box : Vec<&str>= Vec::new();

    for i in 0..=(seq_len-pat_len) {
        let subseq = &data.text[i..i+pat_len];

        let mut diff = 0;

        for s in 0..pat_len {
            if &subseq[s..s+1] != &data.pattern[s..s+1] {
                diff+=1;
            }
        }

        if diff <= data.d { dna_box.push(subseq)}
    }

    Ok(dna_box)

}

pub fn run(content: &Vec<String>) {

    let data = ba1hParser::build(&content).unwrap();

    let results = solve(&data).unwrap();

    for dna_box in results {
        print!("{dna_box} ");
    }
    println!();
}