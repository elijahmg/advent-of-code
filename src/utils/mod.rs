use std::fs;
use std::io::{BufRead, BufReader};

pub fn get_input(path: &str) -> Vec<String> {
    let file = fs::File::open(path)
        .expect("Should have been able to read the file");

    let buf = BufReader::new(file);

    buf
        .lines()
        .map(|l| l.expect("lines should be here"))
        .collect()
}
