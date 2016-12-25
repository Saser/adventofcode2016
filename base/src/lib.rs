//! This crate abstracts away some things that are common for all solutions to Advent of Code
//! problems, for instance by providing a trait that solutions should implement in order to be
//! usable by the `aoc` utility.

extern crate regex;

pub mod coord;
pub mod utils;

use std::str::FromStr;

/// Implementations of this trait should be able to solve Advent of Code problems. Usually an empty
/// struct will suffice, e.g.
///
/// ```
/// # use base::Part;
/// # use base::ProblemSolver;
/// struct Solver;
///
/// impl ProblemSolver for Solver {
///     fn solve(&self, input: &str, part: &Part) -> Result<String, String> {
///         // ...
///         # unimplemented!()
///     }
///     fn solve_file(&self, file_path: &str, part: &Part) -> Result<String, String> {
///         // ...
///         # unimplemented!()
///     }
/// }
/// ```
pub trait ProblemSolver {
    /// Solve the given [`Part`](enum.Part.html) of the problem using `input` as the input string.
    /// If a solution is found, an `Ok` value containing the solution as a text representation
    /// should be returned. If any error occurs, an `Err` value with a description of the error
    /// should be returned.
    fn solve(&self, input: &str, part: &Part) -> Result<String, String>;
}

/// A simple enum to represent either part 1 or part 2 of the problem, as all problems have two
/// parts.
#[derive(Debug, Eq, PartialEq)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(part_str: &str) -> Result<Self, Self::Err> {
        match part_str {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(format!("invalid part specification: {}", part_str)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_from_str() {
        let part1_str = "1";
        let part1 = Part::One;
        assert_eq!(Part::from_str(part1_str).unwrap(), part1);
    }

    #[test]
    fn part2_from_str() {
        let part2_str = "2";
        let part2 = Part::Two;
        assert_eq!(Part::from_str(part2_str).unwrap(), part2);
    }

    #[test]
    fn part_from_str_err() {
        let err_strs = ["", "one", "two", "01", "02", "-1", "-2", "3"];
        for err_str in &err_strs {
            let err = Part::from_str(err_str);
            assert!(err.is_err());
        }
    }
}
