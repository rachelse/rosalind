use crate::mathematics::calculation::EuclideanDistance;

/* Compute the Squared Error Distortion */

fn _parse(line: &String) -> Vec<f32> {
    let parsed : Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
    parsed
}

fn Distortion(centers: &Vec<Vec<f32>>, data : &Vec<Vec<f32>>) -> f32 {
    let mut total_distortion = 0.0;
    for point in data {
        if centers.contains(point) {
            continue
        }
        let mut point_min = f32::MAX;
        for center in centers {
            let tmp_d = EuclideanDistance(&center, &point).unwrap().powf(2.0);
            if tmp_d < point_min {
                point_min = tmp_d;
            }
        }
        total_distortion += point_min;
    }
    return total_distortion/data.len() as f32
}

pub fn run(content: &Vec<String>) {
    let k_m: Vec<usize> = content[0].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let k = k_m[0];
    let m = k_m[1];

    let mut centers : Vec<Vec<f32>> = Vec::new();
    let mut data : Vec<Vec<f32>> = Vec::new();
    let mut i = 1;

    while !content[i].starts_with("-") {
        centers.push(_parse(&content[i]));
        i+=1;
    }
    i+=1;
    while i < content.len() {
        data.push(_parse(&content[i]));
        i+=1
    }

    let answer = Distortion(&centers, &data);
    println!("{answer:?}");
}