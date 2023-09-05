pub fn run(content:&Vec<String>) {
    let seq = &content[0];
    let mut rna = String::new();
    for nt in seq.chars() {
        match nt {
            'T' => {rna.push('U')},
            _ => {rna.push(nt)},
        }
    }
    println!("{rna}");
}