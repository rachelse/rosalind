pub fn run(content: &Vec<String>) {
    let dna = &content[0];
    let pattern = &content[1];
    let k = pattern.len();

    let mut cnt =0;

    for i in 0..=dna.len() - k {
        if &dna[i..i+k] == pattern {
            cnt += 1;
        }
    }

    println!("{cnt}");
}
