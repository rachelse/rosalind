pub fn run(content: &Vec<String>) {
    let n = content.len();
    let k = content[0].len();

    for i in 0..n {
        for j in i+1..n {
            // Compare i, j and get minimum distance
            if &content[i][1..] == &content[j][..k-1] {
                println!("{} -> {}", content[i], content[j]);
            } else if &content[i][..k-1] == &content[j][1..] {
                println!("{} -> {}", content[j], content[i]);
            }
        }
    }
}