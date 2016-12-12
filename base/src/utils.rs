use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn lines_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(&file);
    reader.lines()
        .map(|line| line.unwrap())
        .collect()
}
