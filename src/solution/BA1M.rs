pub fn solve(number: u16, k:u16) {
    let mut quotient = number;
    let mut i: u16 = 0;
    let mut pattern = "".to_owned();

    while pattern.len() < k as usize {
        let nn = quotient % 4;
        match nn {
            0 => {pattern = "A".to_owned() + &pattern},
            1 => {pattern = "C".to_owned() + &pattern},
            2 => {pattern = "G".to_owned() + &pattern},
            3 => {pattern = "T".to_owned() + &pattern},
            _ => ()
        }
        quotient /= 4;
    }

    println!("{pattern}");
}

pub fn run(content : &Vec<String>) {
    let number : u16= content[0].parse().expect("String cannot be parsed into number");
    let k : u16= content[1].parse().expect("String cannot be parsed into number");
    solve(number, k);
}