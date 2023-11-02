/* Compute Limb Lengths in a Tree */
fn _parse(line: &String) -> Vec<usize> {
    let parsed : Vec<usize> = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    parsed
}

fn compute_limb(n:usize, j:usize, D:&Vec<Vec<usize>>) -> usize {
    let mut i_k : Vec<(usize,usize)> = Vec::new();
    for i in 0..n {
        if i==j {continue}
        for k in i+1..n {
            if k == j {continue}
            i_k.push((i,k));
        }
    }

    let mut min_limblength = usize::MAX;
    for (i,k) in i_k {
        let limblength = (D[i][j] + D[j][k] - D[i][k])/2;
        if min_limblength > limblength {
            min_limblength = limblength;   
        }
    }
    min_limblength
}

pub fn run(content:&Vec<String>) {
    let n = content[0].parse::<usize>().unwrap();
    let j = content[1].parse::<usize>().unwrap();
    let mut D : Vec<Vec<usize>> = Vec::new();
    for i in 2..2+n {
        let parsed = _parse(&content[i]);
        D.push(parsed);
    }
    let answer = compute_limb(n, j, &D);
    println!("{answer}");
}