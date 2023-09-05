pub fn run(content: &Vec<String>) {
    let mut hamm_dist = 0;
    let length = &content[0].len();

    for (nt1,nt2) in content[0].chars().zip(content[1].chars()) {
        if nt1 != nt2 {
            hamm_dist +=1;
        }
    }

    println!("{hamm_dist}");
}