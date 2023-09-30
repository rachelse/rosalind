/* Align Two Strings Using Linear Space */
use crate::biology::aminoacid::ScoreMatrix;

static PENALTY: i32= -5;

#[derive(Clone, Debug, Copy)]
pub enum MidEdge {
    V, // Vertical
    H, // Horizontal
    D, // Diagonal
    None
}

pub fn compute_linear(v:&str, wi:char, score: &mut Vec<i32>, track: &mut Vec<MidEdge>) {
    let blosum = ScoreMatrix::build_blosum62();
    let mut prev_score = score.clone();
    score[0] += PENALTY;
    track[0] = MidEdge::H;
    for (i, vi) in v.chars().enumerate() {
        let vertical = score[i] + PENALTY;
        let horizontal = prev_score[i+1] + PENALTY;
        let diagonal = prev_score[i] + blosum.matrix.get(&(wi,vi)).unwrap();

        let (max_idx, max_val) = [diagonal, vertical, horizontal].into_iter().enumerate().max_by(|(_,a),(_,b)|a.cmp(b)).unwrap();
        score[i+1] = max_val;

        if max_idx ==0 {
            track[i+1] = MidEdge::D;
        } else if max_idx == 1 {
            track[i+1] = MidEdge::V;
        } else if max_idx == 2 {
            track[i+1] = MidEdge::H;
        }
    }
}

pub fn initialize_vec(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        vec[i] = i as i32 *PENALTY;
    }
}

pub fn find_middle(v:&String, w: &String, idv: (usize,usize), idw: (usize,usize)) -> (usize,MidEdge) {
    let midw = (idw.0+idw.1)/2;
    let mut i = idv.0;

    // From source to (i,m/2)
    let mut from_src : Vec<i32> = vec![0;idv.1-idv.0+1];
    let mut track_src : Vec<MidEdge> = vec![MidEdge::None;idv.1-idv.0+1];
    initialize_vec(&mut from_src);

    for (i,wi) in w[idw.0..midw].chars().enumerate() {
        compute_linear(&v[idv.0..idv.1], wi, &mut from_src, &mut track_src);
    }

    let v_rev : String = v[idv.0..idv.1].chars().rev().collect();
    let mut to_sink : Vec<i32> = vec![0;idv.1-idv.0+1];
    let mut track_sink : Vec<MidEdge> = vec![MidEdge::None;idv.1-idv.0+1];
    initialize_vec(&mut to_sink);
    for (i, wi) in w[midw..idw.1].chars().rev().enumerate() {
        compute_linear(&v_rev, wi, &mut to_sink, &mut track_sink);
    }
    to_sink = to_sink.into_iter().rev().collect();
    track_sink = track_sink.into_iter().rev().collect();

    let mut src_sink : Vec<i32> = from_src.iter().zip(to_sink).map(|(x,y)| x+y).collect();
    let max_score = src_sink.iter().max().unwrap();
    i = src_sink.iter().position(|val| val == max_score).unwrap();
    if idv == (0,v.len()) && idw==(0,w.len()) {
        println!("{}", max_score);
    }
    (i+idv.0, track_sink[i])
}

pub fn LinearSpaceAlignment(v:&String, w:&String, top:usize, bottom:usize, left:usize, right:usize) -> Vec<(String, String)> {
    if left==right && top == bottom {
        return vec![]
    }
    if left == right {
        // Alignment formed by vertical edges (top-bottom)
        return vec![(v[top..bottom].to_string(), "-".repeat(bottom-top))]
    }

    if top == bottom {
        // Alignment formed by horizontal edges (left-right)
        return vec![("-".repeat(right-left), w[left..right].to_string())]
    }

    let mut midw = (left+right)/2;
    let (mut midv, midedge) = find_middle(&v, &w, (top,bottom), (left,right));

    let mut left_top = LinearSpaceAlignment(&v, &w, top, midv, left, midw);
    let mut aligned = vec![];
    match midedge {
        MidEdge::H => {aligned.push(("-".to_string(), w[midw..midw+1].to_string()));midw+=1;}, 
        MidEdge::V => {aligned.push((v[midv..midv+1].to_string(),"-".to_string()));midv+=1;},
        MidEdge::D => {aligned.push((v[midv..midv+1].to_string(), w[midw..midw+1].to_string()));midw+=1;midv+=1;},
        _ => ()
    }

    let right_bottom = LinearSpaceAlignment(&v, &w, midv, bottom, midw, right);

    return [left_top, aligned, right_bottom].concat()
}

pub fn run(content:&Vec<String>) {
    let v = &content[0];
    let w = &content[1];
    let aligned = LinearSpaceAlignment(&v, &w, 0, v.len(), 0, w.len());

    let mut aligned_v = String::new();
    let mut aligned_w = String::new();

    for (v,w) in aligned {
        aligned_v.push_str(v.as_str());
        aligned_w.push_str(w.as_str());
    }
    println!("{aligned_v}");
    println!("{aligned_w}");
}