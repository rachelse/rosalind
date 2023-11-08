/* Construct the Burrows-Wheeler Transform of a String */

pub fn BWT(text: &String) -> String {
    let mut BW_matrix: Vec<String> = Vec::new();

    for i in 0..text.len() {
        let bwt = bwt_line(&text, i);
        BW_matrix.push(bwt);
    }
    
    BW_matrix.sort();

    let mut last_cols = String::new();

    for text in BW_matrix {
        last_cols.push(text.chars().last().unwrap());
    }

    last_cols
}

fn bwt_line(text: &String, i: usize) -> String {
    let mut bwt = String::new();
    bwt.push_str(&text[i..]);
    bwt.push_str(&text[..i]);
    bwt
}


pub fn run(content: Vec<String>) {
    let text = &content[0];

    let last_cols = BWT(&text);
    println!("{last_cols}");
}