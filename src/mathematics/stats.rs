

pub struct Combination {
    pub n: u8,
    pub r: u8,
    pub case : u8,
    pub combination: Vec<Vec<u8>>
}

impl Combination {
    pub fn new(n:u8, r:u8) -> Self {
        let case = Self::calc_case(n,r);
        Combination {
            n, r,
            case : case,
            combination : Vec::new()
        }
    }

    pub fn fill_combination(&mut self) {
        let mut combinations: Vec<Vec<u8>> = Vec::with_capacity(self.case as usize);

        let mut left = self.r;

        while left > 0 {
            if left == self.r {
                combinations.extend((0..=self.n-self.r).map(|i| vec![i]));
            } else {
                let prev_comb = combinations.clone();
                combinations.clear();
                for v in &prev_comb {
                    let mut new_comb = v.clone();
                    let last = v.last().unwrap();
                    if *last != self.n-1 {
                        for i in last+1..self.n {
                            new_comb.push(i);
                            combinations.push(new_comb.clone());
                            new_comb.pop();
                        }
                    }
                }
            }
            left -= 1;
        }
        self.combination = combinations;
    }

    fn calc_case(n: u8, r:u8) -> u8 {
        let numer :u8 = (n-r+1..=n).product();
        let denom :u8 = (1..=r).product();

        numer/denom
    }

}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_comb() {
        let mut comb = Combination::new(7,3);
        comb.fill_combination();
        assert_eq!(comb.combination.len() as u8, comb.case);
    }
}