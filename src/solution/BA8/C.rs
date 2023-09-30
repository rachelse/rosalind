/* Implement the Lloyd Algorithm for k-Means Clustering */
use std::collections::BTreeMap;
use crate::mathematics::calculation::EuclideanDistance;

fn _parse(line: &String) -> Vec<f32> {
    let parsed : Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect();
    parsed
}

fn CenterToCluster<'a>(k:usize, data: &'a Vec<Vec<f32>>, centers: &Vec<Vec<f32>>) ->  BTreeMap<usize, Vec<&'a Vec<f32>>> {
    let mut cluster : BTreeMap<usize, Vec<&Vec<f32>>> = BTreeMap::new();

    for i in 0..k {
        cluster.insert(i, vec![]);
    }

    for point in data {
        let mut point_min = f32::MAX;
        let mut point_cluster: usize = 0;
        for (i,center) in centers.iter().enumerate() {
            let tmp_d = EuclideanDistance(&center, &point).unwrap();
            if tmp_d < point_min {
                point_min = tmp_d;
                point_cluster = i;
            }
        }
        cluster.get_mut(&point_cluster).unwrap().push(point);
    }
    cluster
}

fn ClusterToCenter(dimension: usize, cluster:&BTreeMap<usize, Vec<&Vec<f32>>>) -> Vec<Vec<f32>>{
    let mut updated_centers: Vec<Vec<f32>> = Vec::new();
    for (i , points) in cluster {
        let n = points.len();
        let mut gravity = vec![0.0;dimension];

        for point in points {
            for (i,p) in point.iter().enumerate() {
                gravity[i]+=p;
            }
        }

        for g in gravity.iter_mut() {
            *g = ((*g/n as f32)*1000.0).round()/1000.0;
        }
        updated_centers.push(gravity);
    }
    updated_centers
}

pub fn run(content:&Vec<String>) {
    let k_m: Vec<usize> = content[0].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let k = k_m[0];
    let m = k_m[1];

    let mut data : Vec<Vec<f32>> = Vec::new();
    let mut centers : Vec<Vec<f32>> = Vec::new();
    let mut converged = false;

    for i in 1..content.len() {
        let parsed = _parse(&content[i]);
        if i <= k {
            centers.push(parsed.clone());
        }
        data.push(parsed.clone());
    }

    // Update centers until converged
    while !converged {
        let cluster = CenterToCluster(k, &data, &centers);
        let updated_centers = ClusterToCenter(m, &cluster);
        if updated_centers == centers {
            converged=true
        }
        centers = updated_centers;
    }

    // Print output
    for center in centers {
        for coordinate in center {
            print!("{coordinate} ");
        }
        println!();
    }
}