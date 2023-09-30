
pub fn EuclideanDistance(p1: &Vec<f32>, p2: &Vec<f32>) -> Result<f32, &'static str> {
    let dim1 = p1.len();
    let dim2 = p1.len();

    if dim1 != dim2 {
        return Err("dimension of p1 and p2 not matched")
    }
    let mut total_d = 0.0;
    for i in 0..dim1 {
        let d = (p1[i]-p2[i]).powf(2.0);
        total_d += d;
    }
    Ok(total_d.sqrt())
}