use std::collections::HashMap;
use phf::phf_map;

pub static AMINOACID : [char;20]= ['A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'Y'];
pub static RNA_CODON1: phf::Map<&'static str, char> = phf_map! {
    "UUU" => 'F', "UCU" => 'S', "UAU" => 'Y',  "UGU" => 'C',
    "UUC" => 'F', "UCC" => 'S', "UAC" => 'Y',  "UGC" => 'C',
    "UUA" => 'L', "UCA" => 'S', "UAA" => '\0', "UGA" => '\0',
    "UUG" => 'L', "UCG" => 'S', "UAG" => '\0', "UGG" => 'W',

    "CUU" => 'L', "CCU" => 'P', "CAU" => 'H',  "CGU" => 'R',
    "CUC" => 'L', "CCC" => 'P', "CAC" => 'H',  "CGC" => 'R',
    "CUA" => 'L', "CCA" => 'P', "CAA" => 'Q',  "CGA" => 'R',
    "CUG" => 'L', "CCG" => 'P', "CAG" => 'Q',  "CGG" => 'R', 

    "AUU" => 'I', "ACU" => 'T', "AAU" => 'N',  "AGU" => 'S',
    "AUC" => 'I', "ACC" => 'T', "AAC" => 'N',  "AGC" => 'S',
    "AUA" => 'I', "ACA" => 'T', "AAA" => 'K',  "AGA" => 'R',
    "AUG" => 'M', "ACG" => 'T', "AAG" => 'K',  "AGG" => 'R',

    "GUU" => 'V', "GCU" => 'A', "GAU" => 'D',  "GGU" => 'G',
    "GUC" => 'V', "GCC" => 'A', "GAC" => 'D',  "GGC" => 'G',
    "GUA" => 'V', "GCA" => 'A', "GAA" => 'E',  "GGA" => 'G',
    "GUG" => 'V', "GCG" => 'A', "GAG" => 'E',  "GGG" => 'G',  
};
pub static DNA_CODON1: phf::Map<&'static str, char> = phf_map! {
    "TTT" => 'F', "TCT" => 'S', "TAT" => 'Y',  "TGT" => 'C',
    "TTC" => 'F', "TCC" => 'S', "TAC" => 'Y',  "TGC" => 'C',
    "TTA" => 'L', "TCA" => 'S', "TAA" => '\0', "TGA" => '\0',
    "TTG" => 'L', "TCG" => 'S', "TAG" => '\0', "TGG" => 'W',

    "CTT" => 'L', "CCT" => 'P', "CAT" => 'H',  "CGT" => 'R',
    "CTC" => 'L', "CCC" => 'P', "CAC" => 'H',  "CGC" => 'R',
    "CTA" => 'L', "CCA" => 'P', "CAA" => 'Q',  "CGA" => 'R',
    "CTG" => 'L', "CCG" => 'P', "CAG" => 'Q',  "CGG" => 'R', 

    "ATT" => 'I', "ACT" => 'T', "AAT" => 'N',  "AGT" => 'S',
    "ATC" => 'I', "ACC" => 'T', "AAC" => 'N',  "AGC" => 'S',
    "ATA" => 'I', "ACA" => 'T', "AAA" => 'K',  "AGA" => 'R',
    "ATG" => 'M', "ACG" => 'T', "AAG" => 'K',  "AGG" => 'R',

    "GTT" => 'V', "GCT" => 'A', "GAT" => 'D',  "GGT" => 'G',
    "GTC" => 'V', "GCC" => 'A', "GAC" => 'D',  "GGC" => 'G',
    "GTA" => 'V', "GCA" => 'A', "GAA" => 'E',  "GGA" => 'G',
    "GTG" => 'V', "GCG" => 'A', "GAG" => 'E',  "GGG" => 'G',  
};

pub struct ScoreMatrix {
    pub matrix: HashMap<(char,char),i32>    
}

impl ScoreMatrix {
    pub fn new() -> Self {
        ScoreMatrix { matrix: HashMap::new() }
    }
    pub fn get_score(&self, aa1:&str, aa2:&str) -> Result<i32,&'static str> {
        if aa1.len()!=1 && aa2.len()!=1 {
            return Err("aminoacid must be 1 letter")
        }
        let char1 = aa1.chars().nth(0).unwrap();
        let char2 = aa2.chars().nth(0).unwrap();
        let value = self.matrix[&(char1,char2)];
        return Ok(value)
    }
    pub fn build_blosum62() -> Self {
        let mut blosum62: HashMap<(char,char),i32> = HashMap::new();
        let blosum  =  
        "  A  C  D  E  F  G  H  I  K  L  M  N  P  Q  R  S  T  V  W  Y
        A  4  0 -2 -1 -2  0 -2 -1 -1 -1 -1 -2 -1 -1 -1  1  0  0 -3 -2
        C  0  9 -3 -4 -2 -3 -3 -1 -3 -1 -1 -3 -3 -3 -3 -1 -1 -1 -2 -2
        D -2 -3  6  2 -3 -1 -1 -3 -1 -4 -3  1 -1  0 -2  0 -1 -3 -4 -3
        E -1 -4  2  5 -3 -2  0 -3  1 -3 -2  0 -1  2  0  0 -1 -2 -3 -2
        F -2 -2 -3 -3  6 -3 -1  0 -3  0  0 -3 -4 -3 -3 -2 -2 -1  1  3
        G  0 -3 -1 -2 -3  6 -2 -4 -2 -4 -3  0 -2 -2 -2  0 -2 -3 -2 -3
        H -2 -3 -1  0 -1 -2  8 -3 -1 -3 -2  1 -2  0  0 -1 -2 -3 -2  2
        I -1 -1 -3 -3  0 -4 -3  4 -3  2  1 -3 -3 -3 -3 -2 -1  3 -3 -1
        K -1 -3 -1  1 -3 -2 -1 -3  5 -2 -1  0 -1  1  2  0 -1 -2 -3 -2
        L -1 -1 -4 -3  0 -4 -3  2 -2  4  2 -3 -3 -2 -2 -2 -1  1 -2 -1
        M -1 -1 -3 -2  0 -3 -2  1 -1  2  5 -2 -2  0 -1 -1 -1  1 -1 -1
        N -2 -3  1  0 -3  0  1 -3  0 -3 -2  6 -2  0  0  1  0 -3 -4 -2
        P -1 -3 -1 -1 -4 -2 -2 -3 -1 -3 -2 -2  7 -1 -2 -1 -1 -2 -4 -3
        Q -1 -3  0  2 -3 -2  0 -3  1 -2  0  0 -1  5  1  0 -1 -2 -2 -1
        R -1 -3 -2  0 -3 -2  0 -3  2 -2 -1  0 -2  1  5 -1 -1 -3 -3 -2
        S  1 -1  0  0 -2  0 -1 -2  0 -2 -1  1 -1  0 -1  4  1 -2 -3 -2
        T  0 -1 -1 -1 -2 -2 -2 -1 -1 -1 -1  0 -1 -1 -1  1  5  0 -2 -2
        V  0 -1 -3 -2 -1 -3 -3  3 -2  1  1 -3 -2 -2 -3 -2  0  4 -3 -1
        W -3 -2 -4 -3  1 -2 -2 -3 -3 -2 -1 -4 -4 -2 -3 -3 -2 -3 11  2
        Y -2 -2 -3 -2  3 -3  2 -1 -2 -1 -1 -2 -3 -1 -2 -2 -2 -1  2  7".to_string();

        for (i,line) in blosum.lines().enumerate() {
            if i == 0 {
                continue
            }
            for (j, b) in line.split_whitespace().enumerate().skip(1) {
                blosum62.insert((AMINOACID[i-1], AMINOACID[j-1]), b.parse::<i32>().expect("error occured building blosum matrix"));
            }
        }

        ScoreMatrix { matrix: blosum62}
    }

    pub fn build_pam250() -> Self {
        let mut pam250: HashMap<(char,char),i32> = HashMap::new();
        let mat  =  
        "  A  C  D  E  F  G  H  I  K  L  M  N  P  Q  R  S  T  V  W  Y
        A  2 -2  0  0 -3  1 -1 -1 -1 -2 -1  0  1  0 -2  1  1  0 -6 -3
        C -2 12 -5 -5 -4 -3 -3 -2 -5 -6 -5 -4 -3 -5 -4  0 -2 -2 -8  0
        D  0 -5  4  3 -6  1  1 -2  0 -4 -3  2 -1  2 -1  0  0 -2 -7 -4
        E  0 -5  3  4 -5  0  1 -2  0 -3 -2  1 -1  2 -1  0  0 -2 -7 -4
        F -3 -4 -6 -5  9 -5 -2  1 -5  2  0 -3 -5 -5 -4 -3 -3 -1  0  7
        G  1 -3  1  0 -5  5 -2 -3 -2 -4 -3  0  0 -1 -3  1  0 -1 -7 -5
        H -1 -3  1  1 -2 -2  6 -2  0 -2 -2  2  0  3  2 -1 -1 -2 -3  0
        I -1 -2 -2 -2  1 -3 -2  5 -2  2  2 -2 -2 -2 -2 -1  0  4 -5 -1
        K -1 -5  0  0 -5 -2  0 -2  5 -3  0  1 -1  1  3  0  0 -2 -3 -4
        L -2 -6 -4 -3  2 -4 -2  2 -3  6  4 -3 -3 -2 -3 -3 -2  2 -2 -1
        M -1 -5 -3 -2  0 -3 -2  2  0  4  6 -2 -2 -1  0 -2 -1  2 -4 -2
        N  0 -4  2  1 -3  0  2 -2  1 -3 -2  2  0  1  0  1  0 -2 -4 -2
        P  1 -3 -1 -1 -5  0  0 -2 -1 -3 -2  0  6  0  0  1  0 -1 -6 -5
        Q  0 -5  2  2 -5 -1  3 -2  1 -2 -1  1  0  4  1 -1 -1 -2 -5 -4
        R -2 -4 -1 -1 -4 -3  2 -2  3 -3  0  0  0  1  6  0 -1 -2  2 -4
        S  1  0  0  0 -3  1 -1 -1  0 -3 -2  1  1 -1  0  2  1 -1 -2 -3
        T  1 -2  0  0 -3  0 -1  0  0 -2 -1  0  0 -1 -1  1  3  0 -5 -3
        V  0 -2 -2 -2 -1 -1 -2  4 -2  2  2 -2 -1 -2 -2 -1  0  4 -6 -2
        W -6 -8 -7 -7  0 -7 -3 -5 -3 -2 -4 -4 -6 -5  2 -2 -5 -6 17  0
        Y -3  0 -4 -4  7 -5  0 -1 -4 -1 -2 -2 -5 -4 -4 -3 -3 -2  0 10".to_string();

        for (i,line) in mat.lines().enumerate() {
            if i == 0 {
                continue
            }
            for (j, b) in line.split_whitespace().enumerate().skip(1) {
                pam250.insert((AMINOACID[i-1], AMINOACID[j-1]), b.parse::<i32>().expect("error occured building blosum matrix"));
            }
        }

        ScoreMatrix { matrix: pam250}
    }

}
