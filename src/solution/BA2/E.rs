pub struct Data {
    k: u8,
    t: u8,
    DNA : Vec<String>
}
impl Data {
    pub fn parse(content: &mut Vec<String>) -> Self {
        let k_t: Vec<u8> = content.get(0).unwrap().split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect();
        content.remove(0);
        let dna = content.to_owned();
        Data {
            k: k_t[0],
            t: k_t[1],
            DNA: dna
        }
    }
}

pub fn convert_base(base:char) -> usize {
    match base {
        'A' => {return 0},
        'C' => {return 1},
        'G' => {return 2},
        'T' => {return 3},
        _ => panic!("No matches from expected bases")
    } 
}

pub fn conver_num(num:usize) -> char {
    match num {
        0 => {return 'A'},
        1 => {return 'C'},
        2 => {return 'G'},
        3 => {return 'T'},
        _ => panic!("No matches from expected numbers")
    }
}

pub fn build_profile(motifs: &Vec<&str>, k: u8) -> Vec<Vec<f32>> {
    let mut cnt = vec![[1_u8;4];k as usize];
    for motif in motifs {
        for (i, nt) in motif.chars().enumerate() {
            let j = convert_base(nt);
            cnt[i][j] += 1;
        }
    }
    let total = 4.0_f32+motifs.len() as f32;
    let mut profile : Vec<Vec<f32>> = cnt.iter().map(|arr| arr.iter()
                                                        .map(|n| *n as f32 / total).collect()).collect();
    profile
}

pub fn find_most_probable<'T>(profile: &Vec<Vec<f32>>, sequence : &'T str) -> &'T str{
    let k = profile.len();

    let mut max_probability = -1.0;
    let mut probable_kmer = "";

    for i in 0..=sequence.len()-k {
        let kmer = &sequence[i..i+k];
        let mut probability = 1.0_f32;
        for (i,nt) in kmer.chars().enumerate() {
            probability *= profile[i][convert_base(nt)];
        }
        if probability > max_probability {
            probable_kmer = kmer;
            max_probability = probability;
        }
    }

    probable_kmer
}

pub fn find_consensus(profile : &Vec<Vec<f32>>) -> String{
    let mut consensus = String::new();
    for pvec in profile {

        if let Some((max_idx, max_val)) = pvec.iter().enumerate()
                            .max_by(|(_,a), (_,b)| a.partial_cmp(b).unwrap()) {
            consensus.push(conver_num(max_idx));
        }
    }
    consensus
}

pub fn calc_score(profile: &Vec<Vec<f32>>, motifs : &Vec<&str>) -> u8 {
    let consensus = find_consensus(profile);
    let mut total_hdist = 0_u8;
    for motif in motifs {
        let cnt =consensus.chars().zip(motif.chars()).filter(|(a,b)| a!=b).count();
        total_hdist += cnt as u8;
    }
    total_hdist
}

pub fn solve(data: &Data) -> Vec<&str> {
    let mut best_motifs : Vec<&str> = data.DNA.iter().map(|seq| &seq[0..data.k as usize]).collect();
    let mut best_profile: Vec<Vec<f32>> = build_profile(&best_motifs, data.k);
    for i in 0..=data.DNA[0].len()-data.k as usize {
        let motif1 = &data.DNA[0][i..i+data.k as usize];
        let mut tmp_motifs = vec![motif1];
        
        for i in 1..data.t as usize{
            // Form profile from motifs Motif(1~t-1)
            let profile = build_profile( &tmp_motifs, data.k);
            let probable_motif = find_most_probable(&profile, &data.DNA[i]);
            tmp_motifs.push(probable_motif);
        }
        let tmp_profile = build_profile(&tmp_motifs, data.k);

        if calc_score(&tmp_profile, &tmp_motifs) < calc_score(&best_profile, &best_motifs) {
            best_motifs = tmp_motifs;
            best_profile = tmp_profile;
        }
    }

    best_motifs
}


pub fn run(content : &mut Vec<String>) {
    let data = Data::parse(content);
    let answer = solve(&data);
    for a in answer {
        println!("{a}");
    }
}