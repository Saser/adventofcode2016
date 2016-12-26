extern crate base;
use base::FromChar;
use base::{Part, ProblemSolver};
use base::coord::{Direction, Position};

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        unimplemented!()
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<Direction>>, String> {
    if input.len() == 0 {
        return Err("parse_input: empty string".to_owned());
    }
    let str_lines = input.split_terminator('\n');
    let mut parsed = Vec::new();
    for str_line in str_lines {
        let directions = base::utils::any_err(str_line.chars()
            .map(Direction::from_char))?;
        parsed.push(directions);
    }
    Ok(parsed)
}

trait Keypad {
    fn press(position: Position) -> Option<String>;
}

struct StandardKeypad;

impl Keypad for StandardKeypad {
    fn press(position: Position) -> Option<String> {
        match position {
            Position(-1, 1) => Some(1.to_string()),
            Position(0, 1) => Some(2.to_string()),
            Position(1, 1) => Some(3.to_string()),
            Position(-1, 0) => Some(4.to_string()),
            Position(0, 0) => Some(5.to_string()),
            Position(1, 0) => Some(6.to_string()),
            Position(-1, -1) => Some(7.to_string()),
            Position(0, -1) => Some(8.to_string()),
            Position(1, -1) => Some(9.to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_single_line_() {
        let input = "UULDDURD\n";
        let expected = vec![vec![Direction::Up,
                                 Direction::Up,
                                 Direction::Left,
                                 Direction::Down,
                                 Direction::Down,
                                 Direction::Up,
                                 Direction::Right,
                                 Direction::Down]];
        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_input_single_line_no_line_terminator() {
        let input = "UULDDURD";
        let expected = vec![vec![Direction::Up,
                                 Direction::Up,
                                 Direction::Left,
                                 Direction::Down,
                                 Direction::Down,
                                 Direction::Up,
                                 Direction::Right,
                                 Direction::Down]];
        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_input_multiple_lines() {
        let input = "ULDRD\nDDUDR\nULDULRDRU\n";
        let expected = vec![vec![Direction::Up,
                                 Direction::Left,
                                 Direction::Down,
                                 Direction::Right,
                                 Direction::Down],
                            vec![Direction::Down,
                                 Direction::Down,
                                 Direction::Up,
                                 Direction::Down,
                                 Direction::Right],
                            vec![Direction::Up,
                                 Direction::Left,
                                 Direction::Down,
                                 Direction::Up,
                                 Direction::Left,
                                 Direction::Right,
                                 Direction::Down,
                                 Direction::Right,
                                 Direction::Up]];
        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_input_multiple_lines_no_trailing_newline() {
        let input = "ULDRD\nDDUDR\nULDULRDRU";
        let expected = vec![vec![Direction::Up,
                                 Direction::Left,
                                 Direction::Down,
                                 Direction::Right,
                                 Direction::Down],
                            vec![Direction::Down,
                                 Direction::Down,
                                 Direction::Up,
                                 Direction::Down,
                                 Direction::Right],
                            vec![Direction::Up,
                                 Direction::Left,
                                 Direction::Down,
                                 Direction::Up,
                                 Direction::Left,
                                 Direction::Right,
                                 Direction::Down,
                                 Direction::Right,
                                 Direction::Up]];
        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_input_err() {
        let err_strs = ["", "U L D", "ASD", "UpLeftDown"];
        for err_str in &err_strs {
            if parse_input(err_str).is_ok() {
                panic!("parse of input did not fail but should have: {}", err_str);
            }
        }
    }
}
