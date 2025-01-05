/* Motzkin Numbers and RNA Secondary Structures */

use std::collections::HashMap;

fn calcMotzNum(n:usize, rna: &String, motz_map : &mut HashMap<usize, u64>) -> u64 {
    if motz_map.contains_key(&n) {
        return *motz_map.get(&n).unwrap()
    } else {
        let (mut motz_left, mut motz_right) = (0,0);
        if motz_map.contains_key(&(n-1)) {
            motz_left = *motz_map.get(&(n-1)).unwrap();
        } else {
            motz_left = calcMotzNum(n-1, rna, motz_map);
        }

        for k in 2..=n {
            let mut tmp_right = 1;
            if motz_map.contains_key(&(k-2)) {
                tmp_right *= *motz_map.get(&(k-2)).unwrap();
            } else {
                tmp_right *= calcMotzNum(k-2, rna, motz_map);
            }

            if motz_map.contains_key(&(n-k)) {
                tmp_right *= *motz_map.get(&(n-k)).unwrap();
            } else {
                tmp_right *= calcMotzNum(n-k, rna, motz_map);
            }
            motz_right += tmp_right;
        }
        motz_map.insert(n, motz_left+motz_right);
        return motz_left + motz_right
    }
}

pub fn run(content: Vec<String>) {
    let rna = &content[1];
    let mut motz_map : HashMap<usize, u64> = HashMap::new();
    motz_map.insert(0, 1);
    motz_map.insert(1, 1);

    let motz_num = calcMotzNum(rna.len(), rna, &mut motz_map);
    println!("{:?}", motz_map);
    println!("{}", motz_num);
}