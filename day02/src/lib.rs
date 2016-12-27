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
        let all_directions = parse_input(input)?;
        match part {
            Part::One => Ok(enter_code(Finger::new(StandardKeypad), all_directions)),
            Part::Two => Ok(enter_code(Finger::new(StupidKeypad), all_directions)),
        }
    }
}

fn enter_code<K>(mut finger: Finger<K>, all_directions: Vec<Vec<Direction>>) -> String
    where K: Keypad
{
    let mut code = String::new();
    for directions in &all_directions {
        for direction in directions {
            finger.walk(*direction);
        }
        code += &finger.press();
    }
    code
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

struct Finger<K: Keypad> {
    keypad: K,
    current_position: Position,
}

impl<K: Keypad> Finger<K> {
    fn new(keypad: K) -> Self {
        Finger {
            keypad: keypad,
            current_position: K::initial_position(),
        }
    }

    fn walk(&mut self, direction: Direction) {
        let next_position = self.current_position.walk(direction);
        self.current_position = match self.keypad.key(next_position) {
            Some(_) => next_position,
            None => self.current_position,
        };
    }

    fn press(&self) -> String {
        self.keypad.key(self.current_position).unwrap()
    }
}

trait Keypad {
    fn key(&self, position: Position) -> Option<String>;

    fn initial_position() -> Position;
}

struct StandardKeypad;

impl Keypad for StandardKeypad {
    fn key(&self, position: Position) -> Option<String> {
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

    fn initial_position() -> Position {
        Position(0, 0)
    }
}

struct StupidKeypad;

impl Keypad for StupidKeypad {
    fn key(&self, position: Position) -> Option<String> {
        match position {
            Position(0, 2) => Some(1.to_string()),

            Position(-1, 1) => Some(2.to_string()),
            Position(0, 1) => Some(3.to_string()),
            Position(1, 1) => Some(4.to_string()),

            Position(-2, 0) => Some(5.to_string()),
            Position(-1, 0) => Some(6.to_string()),
            Position(0, 0) => Some(7.to_string()),
            Position(1, 0) => Some(8.to_string()),
            Position(2, 0) => Some(9.to_string()),

            Position(-1, -1) => Some("A".to_owned()),
            Position(0, -1) => Some("B".to_owned()),
            Position(1, -1) => Some("C".to_owned()),

            Position(0, -2) => Some("D".to_owned()),

            _ => None,
        }
    }

    fn initial_position() -> Position {
        Position(0, 0)
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

    #[test]
    fn test_finger_walk() {
        let mut finger = Finger::new(StandardKeypad);

        finger.walk(Direction::Up);
        assert_eq!(finger.current_position, Position(0, 1));

        finger.walk(Direction::Left);
        assert_eq!(finger.current_position, Position(-1, 1));

        finger.walk(Direction::Down);
        assert_eq!(finger.current_position, Position(-1, 0));

        finger.walk(Direction::Right);
        assert_eq!(finger.current_position, Position(0, 0));
    }

    #[test]
    fn test_finger_walk_too_far() {
        let mut finger = Finger::new(StandardKeypad);

        finger.walk(Direction::Up);
        finger.walk(Direction::Up);
        assert_eq!(finger.current_position, Position(0, 1));

        finger.walk(Direction::Left);
        finger.walk(Direction::Left);
        assert_eq!(finger.current_position, Position(-1, 1));

        finger.walk(Direction::Down);
        finger.walk(Direction::Down);
        finger.walk(Direction::Down);
        assert_eq!(finger.current_position, Position(-1, -1));

        finger.walk(Direction::Right);
        finger.walk(Direction::Right);
        finger.walk(Direction::Right);
        assert_eq!(finger.current_position, Position(1, -1));
    }
}
