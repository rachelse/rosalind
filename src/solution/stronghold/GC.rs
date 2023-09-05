pub fn nt_to_idx(nt:char) -> usize{
    match nt {
        'A' => return 0,
        'C' => return 1,
        'G' => return 1,
        'T' => return 0,
        _ => return usize::MAX
    }
}

pub fn calc_gc(seq : &String) -> f32{
    let mut gc = 0;
    let length = seq.len();
    for nt in seq.chars() {
        let idx = nt_to_idx(nt);
        gc += idx;
    }
    gc as f32 / length as f32 * 100_f32
}

pub fn run(content:&Vec<String>) {
    let n = content.len();
    let mut header = "";
    let mut max_gc = 0.0;

    let mut i = 0;

    let mut tmp_h = "";
    let mut tmp_seq = String::new();

    while i <= n {
        if i == n {
            let tmp_gc = calc_gc(&tmp_seq);
            if tmp_gc > max_gc {
                max_gc = tmp_gc;
                header = tmp_h;
            } 
        } else if &content[i][0..1] == ">" {
            if i != 0{
                let tmp_gc = calc_gc(&tmp_seq);
                if tmp_gc > max_gc {
                    max_gc = tmp_gc;
                    header = tmp_h;
                }
            }
            tmp_h = &content[i][1..];
            tmp_seq.clear();
        } else {
            tmp_seq.push_str(&content[i].trim());
        }
        
        i+=1;
    }
    println!("{header}");
    println!("{max_gc}");
}