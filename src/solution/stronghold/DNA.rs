pub fn nt_to_num(nt:char) -> usize {
    match nt {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => usize::MAX
    }
}


pub fn run(content:&Vec<String>) {
    let mut nt = [0;4];
    let seq = &content[0];
    for i in seq.chars() {
        let idx = nt_to_num(i);
        nt[idx] +=1;
    }

    for num in nt {
        print!("{num} ");
    }
    println!("");
}