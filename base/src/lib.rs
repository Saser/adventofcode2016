use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
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
    fn new() -> Self;

    fn solve(input: &str, part: Part) -> String;

    fn solve_file(file_path: &str, part: Part) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_from_str() {
        let p1_str1 = "one";
        let p1_str2 = "1";
        let p1 = Part::One;
        assert_eq!(Part::from_str(p1_str1).unwrap(), p1);
        assert_eq!(Part::from_str(p1_str2).unwrap(), p1);

        let p2_str1 = "two";
        let p2_str2 = "2";
        let p2 = Part::Two;
        assert_eq!(Part::from_str(p2_str1).unwrap(), p2);
        assert_eq!(Part::from_str(p2_str2).unwrap(), p2);
    }

    #[test]
    fn part_from_str_err() {
        let err1 = "derp";
        let err2 = "0ne";
        let err3 = "3";
        assert!(Part::from_str(err1).is_err());
        assert!(Part::from_str(err2).is_err());
        assert!(Part::from_str(err3).is_err());
    }
}
