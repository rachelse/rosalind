use crate::biology::kmer::*;

pub struct Data {
    seq : String,
    k : u8,
    matrix : [Vec<f32>;4]
}

impl Data {
    pub fn parse(content:&Vec<String>) -> Data {
        let seq = content.get(0).unwrap().clone();

        let k = content.get(1).expect("Failed to get second line").parse::<u8>().expect("k is not parsed into u8");
        let initial_vec = vec![0.0;k as usize];
        let mut matrix = [initial_vec.clone(), initial_vec.clone(), initial_vec.clone(), initial_vec.clone()];
        for i in 2..=5 {
            let one_line: Vec<f32> = content.get(i).unwrap().split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
            let mut j = 0;
            matrix[i-2] = one_line;
        }

        Data {
            seq, k, matrix
        }
    }    
}

pub fn solve(data: &Data) -> String {
    let mut max_kmer = "";
    let mut max_probability :f32 = 0.0;
    // let mut min_vec = Vec::new();

    for i in 0..&data.seq.len()-data.k as usize{
        let kmer = &data.seq[i..i+data.k as usize];
        let mut probability: f32 = 1.0;
        for (i,nt) in kmer.chars().enumerate() {
            match nt {
                'A' => {probability *= data.matrix[0].get(i).unwrap()},
                'C' => {probability *= data.matrix[1].get(i).unwrap()},
                'G' => {probability *= data.matrix[2].get(i).unwrap()},
                'T' => {probability *= data.matrix[3].get(i).unwrap()}, 
                _ => ()
            } 
        }
        if probability > max_probability {
            max_kmer = kmer;
            max_probability = probability;
        }
    }

    max_kmer.to_string()

}

pub fn run(content : &Vec<String>) {
    let data = Data::parse(content);
    let ans = solve(&data);

    println!("{}",ans);
}