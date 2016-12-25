//! This module contains some useful utility functions that can be used by solutions in order to reduce the amount of boilerplate code related to e.g. reading input.

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
    let _ = reader.read_to_string(&mut buf);
    buf
}

pub fn any_err<I, T, E>(mut iterator: I) -> Result<Vec<T>, E>
    where I: Iterator<Item = Result<T, E>>
{
    let mut ok_values = Vec::new();
    while let Some(result) = iterator.next() {
        match result {
            Ok(value) => ok_values.push(value),
            Err(err) => return Err(err),
        };
    }
    Ok(ok_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_err_only_ok() {
        let vals: Vec<Result<_, String>> = vec![Ok(1), Ok(2), Ok(3)];
        let oks = any_err(vals.into_iter()).unwrap();
        assert_eq!(1, oks[0]);
        assert_eq!(2, oks[1]);
        assert_eq!(3, oks[2]);
    }

    #[test]
    fn test_any_err_last_err() {
        let vals = vec![Ok(1), Ok(2), Err("whoops".to_owned())];
        let err = any_err(vals.into_iter());
        assert!(err.is_err());
    }

    #[test]
    fn test_any_err_first_err() {
        let vals = vec![Err("whoops".to_owned()), Ok(2), Ok(3)];
        let err = any_err(vals.into_iter());
        assert!(err.is_err());
    }

    #[test]
    fn test_any_err_middle_err() {
        let vals = vec![Ok(1), Err("whoops".to_owned()), Ok(3)];
        let err = any_err(vals.into_iter());
        assert!(err.is_err());
    }
}
