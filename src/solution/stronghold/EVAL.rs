/* Expected Number of Restriction Sites */
fn calcPossibility(gc:f64, dna:&String) -> f64 {
    let mut possibility = 1.0;
    for nt in dna.chars() {
        match nt {
            'A' | 'T' => {possibility*=(0.5-gc/2.0)},
            'G' | 'C' => {possibility*=gc/2.0},
            _ => {}       
        }
    }
    possibility
}

fn eval(n: usize, s:&String, arrA: &Vec<f64>) {
    let mut arrB : Vec<f64> = Vec::new();
    for content in arrA {
        let gc = content;
        let p = calcPossibility(*gc, s);
        print!("{:.3} ", p*(n-s.len()+1) as f64);
    }
    println!();
}

pub fn run(content : &Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    let s = &content[1];
    let arrA : Vec<f64> = content[2].split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect(); //GC content
    eval(n, s, &arrA);
}