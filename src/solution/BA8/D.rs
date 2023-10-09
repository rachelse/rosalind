/* Implement the Soft k-Means Clustering Algorithm */
use crate::mathematics::calculation::EuclideanDistance;

fn _parse(line: &String) -> Vec<f32> {
    let parsed : Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
    parsed
}

fn _partition(point: &Vec<f32>, center:&Vec<f32>, stiffness:f32) -> f32{
    use std::f32::consts::E;
    let dist = EuclideanDistance(&point, &center).unwrap();
    1.0/E.powf(stiffness*dist)
}

fn Estep<'a>(k:usize, data: &'a Vec<Vec<f32>>, centers: &Vec<Vec<f32>>, stiffness:f32) -> Vec<Vec<f32>> {
    /* Centers to Soft clusters */
    let mut HiddenMatrix: Vec<Vec<f32>> = vec![vec![0.0;k];data.len()];

    for (j,point) in data.iter().enumerate() {
        let mut distance_matrix = vec![0.0;k];
        for (i,center) in centers.iter().enumerate() {
            distance_matrix[i] = _partition(&point, &center, stiffness);
        }
        let sum :f32 = distance_matrix.iter().sum();
        for (i, val) in distance_matrix.iter().enumerate() {
            HiddenMatrix[j][i] = val/sum; // convert into responsibility 
        }
    }
    HiddenMatrix
}

fn Mstep(dimension: usize, Data:&Vec<Vec<f32>>, HiddenMatrix:&Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut weighted_centers: Vec<Vec<f32>> = Vec::new();
    let k = HiddenMatrix[0].len();
    let n = HiddenMatrix.len();

    for i in 0..k {
        let mut numerator = vec![0.0;dimension];
        let mut denominator = 0.0;
        for j in 0..n {
            let hmi = HiddenMatrix[j][i];
            for (dim, dai) in Data[j].iter().enumerate() {
                numerator[dim]+=hmi*dai;
            }
            denominator+=hmi;
        }
        for elem in numerator.iter_mut() {
            *elem /= denominator;
        }
        weighted_centers.push(numerator);
    }
    weighted_centers
}

pub fn run(content:&Vec<String>) {
    let k_m: Vec<usize> = content[0].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let k = k_m[0];
    let m = k_m[1];
    let b = content[1].parse::<f32>().unwrap(); // stiffness(beta)

    let mut data : Vec<Vec<f32>> = Vec::new();
    let mut centers : Vec<Vec<f32>> = Vec::new();

    for i in 2..content.len() {
        let parsed = _parse(&content[i]);
        data.push(parsed.clone());
    }

    /* Initialize centers with first k points */
    for i in 0..k {
        centers.push(data[i].clone());
    }

    /* Update centers applying soft k-means clustering algorithm */
    for i in 0..100 {
        let hidden_matrix = Estep(k, &data, &centers, b);
        centers = Mstep(m, &data, &hidden_matrix);
    }

    /* Print output */
    for center in centers {
        for coordinate in center {
            print!("{coordinate:.3} ");
        }
        println!();
    }
}