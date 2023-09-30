/* Find the Length of a Longest Path in a Manhattan-like Grid */

use std::cmp;

pub fn parse_line(line:&String) -> Vec<usize> {
    let parsed : Vec<usize>= line.split_whitespace().map(|x| x.parse::<usize>().expect("string not parsed into usize")).collect();
    parsed
}

pub struct Map {
    pub n: usize,
    pub m: usize,
    pub south : Vec<Vec<usize>>,
    pub east : Vec<Vec<usize>>,
    pub dist : Vec<Vec<usize>>
}

impl Map {
    pub fn new(n:usize, m:usize, content:&[String]) -> Self {
        let mut map = Map {
                n:n,m:m,
                south : vec![vec![0;m+1];n],
                east : vec![vec![0;m];n+1],
                dist : vec![vec![0;m+1];n+1]
                };
        map.fill(&content);
        map.initialize_dist();
        map
    }

    fn fill(&mut self, content:&[String]) {
        let mut south = true;

        for i in 0..content.len() {
            match content[i].as_str() {
                "-" => {south=false;}
                _ => {
                    if south {
                        self.south[i] = parse_line(&content[i]);
                    } else {
                        self.east[i - self.n -1] = parse_line(&content[i]);
                    }
                }
            }
        }
    }

    fn initialize_dist(&mut self) {
        for n in 1..=self.n {
            let val = self.dist_from_upper(0, n);
            self.set_dist(0, n, val);
        }

        for m in 1..=self.m {
            let val = self.dist_from_left(m, 0);
            self.set_dist(m,0,val);
        }
    }

    pub fn dist_from_left(&self, m: usize, n: usize) -> usize {
        if m == 0 {
            return 0
        }
        self.get_dist(m-1, n) + self.east[n][m-1]
    }

    pub fn dist_from_upper(&self, m: usize, n: usize) -> usize {
        if n == 0 {
            return 0
        }
        self.get_dist(m, n-1) + self.south[n-1][m]
    }

    pub fn set_dist(&mut self, m: usize, n: usize, val: usize) {
        self.dist[n][m] = val;
    }

    pub fn get_dist(&self, m: usize, n: usize) -> usize {
        self.dist[n][m]
    }
}

pub fn ManhattanTourist(map: &mut Map) -> usize {
    for m in 1..=map.m {
        for n in 1..=map.n {
            let val = cmp::max(map.dist_from_left(m, n), map.dist_from_upper(m, n));
            map.set_dist(m,n, val);
        }
    }
    map.dist[map.n][map.m]
}

pub fn run(content: &Vec<String>) {
    let n_m = parse_line(&content[0]);
    let mut map = Map::new(n_m[0], n_m[1], &content[1..]);
    let longest = ManhattanTourist(&mut map);
    println!("{longest}");
}