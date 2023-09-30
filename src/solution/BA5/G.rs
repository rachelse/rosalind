/* Compute the Edit Distance Between Two Strings */

// use crate::biology::calc_editdist;
use std::collections::HashMap;

pub fn calc_editdist<'T>(s1: &'T str, s2: &'T str, i1:usize, i2: usize, distmap :&mut HashMap<(usize,usize), usize>) -> usize{
    if distmap.contains_key(&(i1,i2)) {
        return *distmap.get(&(i1,i2)).unwrap()
    } 

    if s1.len() == i1 || s2.len() == i2 {
        let remain = usize::abs_diff(s1.len()-i1, s2.len()-i2);
        if !distmap.contains_key(&(i1,i2)) {
            distmap.insert((i1,i2), remain);
        }
        return remain
    } 
    let mut dist = 0;
    if s1[i1..i1+1] == s2[i2..i2+1] { // match
        dist = calc_editdist(&s1, &s2,i1+1,i2+1, distmap);
        distmap.entry((i1,i2)).or_insert(dist);
    } else {
        let insertion = calc_editdist(&s1, &s2,i1,i2+1, distmap);
        let deletion = calc_editdist(&s1, &s2, i1+1,i2,distmap);
        let substitution = calc_editdist(&s1, &s2,i1+1,i2+1, distmap);
        dist = [insertion, deletion, substitution].into_iter().min().unwrap()+1;
        distmap.insert((i1,i2),dist);
    }    
    return dist
}

pub fn run(content: &Vec<String>) {
    let s1 = &content[0];
    let s2 = &content[1];
    let mut distmap: HashMap<(usize, usize), usize> = HashMap::new();
    let edit_dist = calc_editdist(&s1[..],&s2[..],0,0, &mut distmap);
    println!("{edit_dist}");
}