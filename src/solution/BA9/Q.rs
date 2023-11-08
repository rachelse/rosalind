/* Construct the Partial Suffix Array of a String */

pub fn SuffixArray(text: &String) -> Vec<usize> {
    let mut suffix_array = Vec::new();
    for i in 0..text.len() {
        let suffix = format!("{}{}:{i}", &text[i..], &text[..i]);
        suffix_array.push(suffix);
    }
    suffix_array.sort();

    let mut suffix_idx = Vec::new();
    for suffix in suffix_array {
        let idx = suffix.split(":").nth(1).unwrap().parse::<usize>().unwrap();
        suffix_idx.push(idx);
    }
    suffix_idx
}

pub fn run(content: Vec<String>) {
    let text= &content[0];
    let k = content[1].parse::<usize>().unwrap();

    let suffix_array = SuffixArray(&text);

    for (i, suffix_idx) in suffix_array.iter().enumerate() {
        if suffix_idx % k == 0 {
            println!("{i},{suffix_idx}");
        }
    }
}