pub fn run(content: &Vec<String>) {
    let n = content.len();
    let k = content[0].len();
    let mut predecessor: Vec<Option<usize>> = vec![None;n];
    let mut successor: Vec<Option<usize>> = vec![None;n];

    for i in 0..n {
        for j in i+1..n {
            // Compare i, j and get minimum distance
            if &content[i][1..] == &content[j][..k-1] {
                predecessor[j] = Some(i);
                successor[i] = Some(j);
            } else if &content[i][..k-1] == &content[j][1..] {
                predecessor[i] = Some(j);
                successor[j] = Some(i);
            }
        }
    }

    let header = predecessor.iter().position(|&x| x==None ).unwrap();
    let mut answer = content[header].clone();
    let mut next = successor[header];
    while next != None {
        let next_char = &content[next.unwrap()][k-1..k];
        answer.push_str(next_char);
        next = successor[next.unwrap()];
    }

    println!("{answer}");
}