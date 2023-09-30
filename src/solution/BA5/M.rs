/* Find a Highest-Scoring Multiple Sequence Alignment */

#[derive(Clone, Debug, PartialEq)]
pub struct Tracker(usize,usize,usize);

fn initializeMatrix<T: Clone>(val:T, n1:usize, n2:usize, n3:usize) -> Vec<Vec<Vec<T>>>{
    let d1_matrix = vec![val;n1+1];
    let d2_matrix = vec![d1_matrix;n2+1];
    let d3_matrix = vec![d2_matrix;n3+1];
    d3_matrix
}

fn get_score(s1: &str, s2: &str, s3:&str) -> u32 {
    if (s1==s2) && (s2==s3) {
        return 1
    }
    return 0
}

fn fill_alignment(alignment:&mut String, prev: usize, current:usize, sequence: &String) {
    if current == 0 { 
        return
    } else if prev == current {
        alignment.push('-');
    } else {
        for c in sequence[prev..current].chars().rev().collect::<String>().chars() {
            alignment.push(c);
        }   
    }
}

fn align(s1: &String, s2: &String, s3: &String) -> Vec<Vec<Vec<Tracker>>> {
    let mut score = initializeMatrix(0_u32,s1.len(), s2.len(), s3.len());
    let mut track = initializeMatrix(Tracker(0,0,0), s1.len(), s2.len(), s3.len());

    for i1 in 1..=s1.len() {
        for i2 in 1..=s2.len() {
            for i3 in 1..=s3.len() {
                let scores = [score[i3-1][i2-1][i1-1]+ get_score(&s1[i1-1..i1], &s2[i2-1..i2], &s3[i3-1..i3]),
                                        score[i3-1][i2-1][i1], score[i3-1][i2][i1-1], score[i3][i2-1][i1-1],
                                        score[i3][i2][i1-1], score[i3][i2-1][i1], score[i3-1][i2][i1]];
                let max_score = scores.iter().max().unwrap().to_owned();
                let max_idx = scores.iter().position(|&x| x == max_score);

                score[i3][i2][i1] = max_score;
                match max_idx {
                    Some(0) => {track[i3][i2][i1] = Tracker(i1-1,i2-1,i3-1)},
                    Some(1) => {track[i3][i2][i1] = Tracker(i1,i2-1,i3-1)}, 
                    Some(2) => {track[i3][i2][i1] = Tracker(i1-1,i2,i3-1)},
                    Some(3) => {track[i3][i2][i1] = Tracker(i1-1,i2-1,i3)},
                    Some(4) => {track[i3][i2][i1] = Tracker(i1-1,i2,i3)},
                    Some(5) => {track[i3][i2][i1] = Tracker(i1,i2-1,i3)},
                    Some(6) => {track[i3][i2][i1] = Tracker(i1,i2,i3-1)},
                    _ => ()
                }
            }
        }
    }
    println!("{}", score[s3.len()][s2.len()][s1.len()]);
    return track
}

fn track(s1:&String, s2:&String, s3:&String, tracker : &Vec<Vec<Vec<Tracker>>>) {
    let mut aligned1 = String::new();
    let mut aligned2 = String::new();
    let mut aligned3 = String::new();

    let mut pos = &Tracker(s1.len(), s2.len(), s3.len());
    
    while pos != &Tracker(0,0,0) {
        let tracked = &tracker[pos.2][pos.1][pos.0];
        fill_alignment(&mut aligned1, tracked.0, pos.0, &s1);
        fill_alignment(&mut aligned2, tracked.1, pos.1, &s2);
        fill_alignment(&mut aligned3, tracked.2, pos.2, &s3);
        // Fill remainder
        if tracked == &Tracker(0,0,0) {
            let max = [pos.0, pos.1, pos.2].iter().max().unwrap().clone();
            for i in 0..(max-pos.0) { 
                aligned1.push('-');
            }
            for i in 0..(max-pos.1) {
                aligned2.push('-');
            }
            for i in 0..(max-pos.2) {
                aligned3.push('-');
            }
        }
        pos = tracked;
    }



    aligned1 = aligned1.chars().rev().collect();
    aligned2 = aligned2.chars().rev().collect();
    aligned3 = aligned3.chars().rev().collect();

    println!("{aligned1}");
    println!("{aligned2}");
    println!("{aligned3}");
}

pub fn run(content:&Vec<String>) {
    let s1 = &content[0];
    let s2 = &content[1];
    let s3 = &content[2];

    let tracker = align(&s1, &s2, &s3);
    track(&s1, &s2, &s3, &tracker);
}