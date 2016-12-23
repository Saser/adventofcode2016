pub mod utils;

use std::str::FromStr;

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

pub trait ProblemSolver {
    fn solve(&self, input: &str, part: &Part) -> Result<String, String>;

    fn solve_file(&self, file_path: &str, part: &Part) -> Result<String, String>;
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
