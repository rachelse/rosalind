use crate::kmer::calc_hammdist;

pub fn run(content: &Vec<String>) {
    let pattern = content[0].as_str();
    let dna: Vec<&str> = content[1].split_whitespace().collect();
    let mut distance = 0_u8 ;

    for i in 0..dna.len() {
        let mut hamm_dist = std::u8::MAX;
        let dna_string = dna[i];
        for i in 0..=dna_string.len()-pattern.len() {
            let kmer = &dna_string[i..i+pattern.len()];
            let tmp_dist = calc_hammdist(pattern,kmer);
            if hamm_dist > tmp_dist {
                hamm_dist = tmp_dist;
            }
        }
        distance += hamm_dist;
    }
    println!("{distance}");
}