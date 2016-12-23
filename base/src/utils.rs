//! This module contains some useful utility functions that can be used by solutions in order to
//! reduce the amount of boilerplate code related to e.g. reading input.

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Takes a file path and returns a `Vec` of all the lines, without any trailing newline
/// characters. Assumes that all lines in the file are readable as strings.
pub fn lines_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(&file);
    reader.lines()
        .map(|line| line.unwrap())
        .collect()
}
