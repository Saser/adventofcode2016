//! This module contains some useful utility functions that can be used by solutions in order to
//! reduce the amount of boilerplate code related to e.g. reading input.

use std::fs::File;
use std::io::{BufReader, Read};

/// Takes a file path and returns a `Vec` of all the lines, without any trailing newline
/// characters. Assumes that all lines in the file are readable as strings.
pub fn lines_from_file(file_path: &str) -> Vec<String> {
    read_file_as_string(file_path).split('\n').map(str::to_owned).collect()
}

/// Takes a file path and reads the contents of that file, assuming that it is a text file. The
/// read contents are returned as a new `String`.
pub fn read_file_as_string(file_path: &str) -> String {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(&file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf);
    buf
}
