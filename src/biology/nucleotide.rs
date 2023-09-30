use std::io;

pub static NUCLEOTIDE : [char;4]= ['A', 'C', 'G', 'T'];

pub fn reverse_nucleotide(nn:char) -> Result<char, &'static str> {
    match nn {
        'A' => {return Ok('T')},
        'T' => {return Ok('A')},
        'G' => {return Ok('C')},
        'C' => {return Ok('G')},
        _ => {Err("no complement exists")}
    }
}

pub fn reverse_sequence(seq: &str) -> String {
    let mut reverse_complement = String::new();
    for nn in seq.chars().rev() {
        reverse_complement.push(reverse_nucleotide(nn).expect("no reverse nucleotide found"));
    }
    reverse_complement
}