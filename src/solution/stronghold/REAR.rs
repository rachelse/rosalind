/* Reversal Distance */

use std::collections::HashMap;


fn checkMarker(qry: &Vec<usize>, tar:&Vec<usize>, marker: &mut Vec<bool>) {
    for (i,(q,t)) in qry.iter().zip(tar).enumerate() {
        if q == t {
            let m = marker.get_mut(i).unwrap();
            *m = true;
        }
    }
}

fn calcReversalDistance(qry: &Vec<usize>, tar:&Vec<usize>, marker: &mut Vec<bool>) {

}

pub fn run(content : Vec<String>) {
    for i in 0..5 {
        let mut qry : Vec<usize> = content[i*3].split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut tar : Vec<usize> = content[i*3+1].split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut marker = vec![false;qry.len()];
        checkMarker(&qry, &tar, &mut marker);

        let mut rdist = 0;

        println!("{i} {marker:?}");
    }
}