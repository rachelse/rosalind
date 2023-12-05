pub fn get_max_f64(score: &Vec<f64>) -> usize {
    let mut max_s = 0.0;
    let mut max_i = 0;
    for (i,s) in score.iter().enumerate() {
        if *s > max_s {
            max_s = *s;
            max_i = i;
        }
    }
    max_i
}