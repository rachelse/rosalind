/* Find a Highest-Scoring Fitting Alignment of Two Strings */

static MATCH : i32 = 1;
static PENALTY : i32 = -1;

#[derive(Clone, PartialEq, Debug)]
pub enum Track{
    None,
    Left,
    Up,
    Diagonal,
}

pub fn initialize_matrix(matrix: &mut Vec<Vec<Option<i32>>>) {
    let lenw = matrix[0].len();
    for i in 0..lenw {
        matrix[0][i] = Some(0);
    }
}

pub fn get_direction(left: i32, up:i32, diagonal:i32) -> Track {
    let argmax = [&diagonal, &up, &left].iter().enumerate().max_by(|&(_,a),&(_,b)|a.cmp(b)).map(|(idx,_)| idx);
    match argmax {
        Some(0) => {return Track::Diagonal},
        Some(1) => {return Track::Up},
        Some(2) => {return Track::Left},
        _ => {return Track::None} 
    }
}

pub fn fit_alignment(v:&String, w: &String, score: &mut Vec<Vec<Option<i32>>>, track: &mut Vec<Vec<Track>>) -> (usize,usize) {
    let lenv = v.len();
    let lenw = w.len();

    let mut max_pos = (0_usize,0_usize);
    for iw in 1..=lenw {
        for iv in iw..=lenv-lenw+iw {
            // println!("{iw} {iv}");
            let mut diagonal = 0;
            let mut left = i32::MIN; 
            let mut up = i32::MIN; 

            // Match or Substitution
            if &w[iw-1..iw] == &v[iv-1..iv] {
                diagonal = score[iw-1][iv-1].expect("expected i32 for score matrix")+MATCH;
            } else {
                diagonal = score[iw-1][iv-1].expect("expected i32 for score matrix")+PENALTY;
            }

            if iv != iw { // Insertion: Ignore if left value is None
                left = score[iw][iv-1].expect("expected i32 for score matrix")+PENALTY;
            }

            if iv != lenv-lenw+iw {
                up = score[iw-1][iv].expect("expected i32 for score matrix")+PENALTY;
            }

            let direction = get_direction(left, up, diagonal);
            match direction {
                Track::Left => {score[iw][iv] = Some(left)},
                Track::Up => {score[iw][iv] = Some(up)},
                Track::Diagonal => {score[iw][iv] = Some(diagonal)},
                _ => ()
            }
            track[iw][iv] = direction;

            if (iw==lenw) && (score[iw][iv] > score[max_pos.1][max_pos.0]) {
                max_pos = (iv,iw);
            }
        }
    }

    return max_pos
}

pub fn track(start:(usize,usize), score_matrix : &Vec<Vec<Option<i32>>>, track_matrix: &Vec<Vec<Track>>, v:&String,w:&String) {
    let max_score = score_matrix[start.1][start.0].unwrap();
    println!("{}",max_score);

    let (mut vi,mut wi) = start;
    let mut aligned_v = String::new();
    let mut aligned_w = String::new();
    while track_matrix[wi][vi] != Track::None {

        match track_matrix[wi][vi] {
                Track::Left => {aligned_v.push_str(&v[vi-1..vi]); aligned_w.push('-');vi-=1;},
                Track::Up => {aligned_w.push_str(&w[wi-1..wi]); aligned_v.push('-');wi-=1;}, 
                Track::Diagonal => {aligned_v.push_str(&v[vi-1..vi]);aligned_w.push_str(&w[wi-1..wi]);vi-=1;wi-=1 },
                _ => break
            }
    }

    aligned_v = aligned_v.chars().rev().collect();
    aligned_w = aligned_w.chars().rev().collect();
    println!("{aligned_v}");
    println!("{aligned_w}");
}

pub fn run(content:&Vec<String>) {
    let v = &content[0];
    let w = &content[1];
    let lenv = v.len();
    let lenw = w.len();

    let mut score_matrix: Vec<Vec<Option<i32>>> = vec![vec![None;lenv+1];lenw+1];
    let mut track_matrix : Vec<Vec<Track>> = vec![vec![Track::None;lenv+1];lenw+1];
    initialize_matrix(&mut score_matrix);
    let start_pos = fit_alignment(&v, &w, &mut score_matrix, &mut track_matrix);

    track(start_pos, &score_matrix, &track_matrix, &v, &w)
}