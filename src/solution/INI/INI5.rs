/*
 * File: INI5.rs
 * Project: INI
 * File Created: 12th Jan 2025
 * Author: Rachel Seongeun Kim (seamustard52@gmail.com)
 * -----
 * Copyright: Rachel Seongeun Kim
 */

pub fn run(content: Vec<String>) {
    for i in (1..content.len()).step_by(2) {
        println!("{}", content[i]);
    }
}