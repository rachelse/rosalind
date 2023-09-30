/* Find a Topological Ordering of a DAG */

use std::collections::HashMap;

pub fn _parse_line(line:&String) -> (u8,Vec<u8>) {
    let nodes: Vec<&str> = line.split(" -> ").into_iter().collect();
    let n1 = nodes[0].parse::<u8>().unwrap();
    let n2s: Vec<u8> = nodes[1].split(",").into_iter().map(|x| x.parse::<u8>().unwrap()).collect();

    (n1, n2s)
}

pub fn TopologicalOrdering(incoming: &mut HashMap<u8,Vec<u8>>, outgoing: &mut HashMap<u8,Vec<u8>>) -> Vec<u8> {
    let mut list: Vec<u8> = Vec::new();
    let mut candidates : Vec<u8> = Vec::new();

    for (node,v) in &mut *incoming {
        if v.len() == 0 {
            candidates.push(*node);
        }
    }

    while !candidates.is_empty() {
        let candidate = candidates.remove(0);
        list.push(candidate);

        incoming.remove(&candidate);
        for n2 in &outgoing[&candidate] {
            let mut income_n2 = incoming.get_mut(n2);

            income_n2.unwrap().retain(|&x| x != candidate);

            if incoming[&n2].len() == 0 {
                candidates.push(*n2);
            }
        }
        outgoing.remove(&candidate);
    }

    list
}

pub fn run(content: &Vec<String>) {
    let mut outgoing : HashMap<u8,Vec<u8>> = HashMap::new();
    let mut incoming : HashMap<u8,Vec<u8>> = HashMap::new();

    for line in content {
        let (n1,n2) = _parse_line(&line);

        let mut out1 = outgoing.entry(n1).or_default();
        *out1 = n2.clone();
        for n in n2 {
            let mut out2 = outgoing.entry(n).or_default();
            let mut inc2 = incoming.entry(n).or_default();
            inc2.push(n1);
        }
        let mut inc = incoming.entry(n1).or_default();
    }

    let list = TopologicalOrdering(&mut incoming, &mut outgoing);

    // Print output
    let mut iter = list.iter();
    print!("{}", iter.next().unwrap());
    for e in iter {
        print!(", {e}")
    }
    println!();
}