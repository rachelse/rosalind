use crate::mathematics::calculation::EuclideanDistance;

/* Implement FarthestFirstTraversal */

fn _parse(line: &String) -> Vec<f32> {
    let parsed : Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
    parsed
}

fn findMaximizingDataPoint(centers: &Vec<Vec<f32>>, data : &Vec<Vec<f32>>) -> Result<Vec<f32>,&'static str> {
    let mut max_dp= None;
    let mut max_d = 0.0_f32;
    for point in data {
        if centers.contains(point) {
            continue
        }
        let mut point_min = f32::MAX;
        for center in centers {
            let tmp_d = EuclideanDistance(&center, &point).unwrap();
            if tmp_d < point_min {
                point_min = tmp_d;
            }
        }
        if point_min > max_d {
            max_d = point_min;
            max_dp = Some(point);
        }
    }
    if max_dp == None {
        return Err("datapoint was not initialized")
    } else {
        return Ok(max_dp.unwrap().clone())
    }
}

fn FarthestFirstTraversal(data:&Vec<Vec<f32>>, k: usize) -> Vec<Vec<f32>> {
    let mut centers : Vec<Vec<f32>> = Vec::new();
    centers.push(data[0].clone());

    while centers.len() < k {
        let datapoint: Vec<f32> = findMaximizingDataPoint(&centers, &data).unwrap();
        centers.push(datapoint);
    }

    centers
}

pub fn run(content: &Vec<String>) {
    let k_m: Vec<usize> = content[0].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let k = k_m[0];
    let m = k_m[1];

    let mut data : Vec<Vec<f32>> = Vec::new();
    for i in 1..content.len() {
        data.push(_parse(&content[i]));
    }

    let answer = FarthestFirstTraversal(&data, k);
    for point in answer {
        for p in point {
            print!("{p:?} ");
        }
        println!()
    }
}