/* Compute the Probability of a Hidden Path */
pub fn _parse_line(line : &String) -> Vec<f32> {
    let splitted: Vec<f32> = line.split_whitespace().skip(1).map(|x| x.parse::<f32>().unwrap()).collect();
    splitted
}

pub fn idx_to_state(states:&Vec<char>, idx: usize) -> char{
    states[idx]
}

pub fn state_to_idx(states: &Vec<char>, state: char) -> usize {
    let state_idx = states.iter().position(|&x| x==state).unwrap();
    state_idx
}

pub fn computeProbability(hidden_path: &String, states:Vec<char>, transition : Vec<Vec<f32>>) -> f32 {
    let mut prob = 1.0 / states.len() as f32;
    let mut prev_state = state_to_idx(&states, hidden_path.chars().nth(0).unwrap());
    for c in hidden_path[1..].chars() {
        let current_state = state_to_idx(&states, c);
        prob *= transition[prev_state][current_state];
        prev_state = current_state;
    }
    
    prob
}

pub fn run(content: Vec<String>) {
    let hidden_path = &content[0];
    let states: Vec<char> = content[2].chars().filter(|&x| x!=' ' && x!='\t').collect();
    let mut transition_matrix = Vec::new();

    for i in 5..content.len() {
        transition_matrix.push(_parse_line(&content[i]));
    }
    
    let probability = computeProbability(hidden_path, states, transition_matrix);
    println!("{probability}");
}