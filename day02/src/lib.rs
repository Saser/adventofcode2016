extern crate base;
use base::{Part, ProblemSolver};

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: &Part) -> Result<String, String> {
        let lines = input.split('\n')
            .collect::<Vec<&str>>();
        match *part {
            Part::One => solve_part_one(&lines),
            Part::Two => solve_part_two(&lines),
        }
    }

    fn solve_file(&self, file_path: &str, part: &Part) -> Result<String, String> {
        let lines = base::utils::lines_from_file(file_path);
        let lines = lines.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
        match *part {
            Part::One => solve_part_one(&lines),
            Part::Two => solve_part_two(&lines),
        }
    }
}

// Here starts the actual solution

use std::str::FromStr;

fn solve_part_one(input_lines: &[&str]) -> Result<String, String> {
    let mut keypad = Keypad::new();
    let mut answer = String::new();

    for line in input_lines {
        for c in line.chars() {
            let result = Movement::from_str(&c.to_string());
            match result {
                Ok(mvmt) => keypad.do_move(&mvmt),
                Err(err) => return Err(err),
            }
        }
        answer.push_str(&keypad.press().to_string());
    }

    Ok(answer)
}

fn solve_part_two(input_lines: &[&str]) -> Result<String, String> {
    unimplemented!()
}

#[derive(Debug, Eq, PartialEq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Movement::Up),
            "D" => Ok(Movement::Down),
            "L" => Ok(Movement::Left),
            "R" => Ok(Movement::Right),
            _ => Err(format!("not a valid movement: {}", s)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct Keypad {
    position: Position,
}

impl Keypad {
    fn new() -> Keypad {
        Keypad { position: Position { x: 0, y: 0 } }
    }

    fn do_move(&mut self, movement: &Movement) {
        match *movement {
            Movement::Up => self.move_up(),
            Movement::Down => self.move_down(),
            Movement::Left => self.move_left(),
            Movement::Right => self.move_right(),
        }
    }

    fn move_up(&mut self) {
        if self.position.y < 1 {
            self.position.y += 1
        }
    }

    fn move_down(&mut self) {
        if self.position.y > -1 {
            self.position.y -= 1
        }
    }

    fn move_left(&mut self) {
        if self.position.x > -1 {
            self.position.x -= 1
        }
    }

    fn move_right(&mut self) {
        if self.position.x < 1 {
            self.position.x += 1
        }
    }

    fn press(&self) -> u32 {
        let Position { x, y } = self.position;
        (1 + -(y - 1) * 3 + (x + 1)) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_up() {
        let mut keypad = Keypad::new();
        keypad.position.y = -1;

        keypad.do_move(&Movement::Up);
        assert_eq!(keypad.position.y, 0);
        keypad.do_move(&Movement::Up);
        assert_eq!(keypad.position.y, 1);
        keypad.do_move(&Movement::Up);
        assert_eq!(keypad.position.y, 1);
    }

    #[test]
    fn test_move_down() {
        let mut keypad = Keypad::new();
        keypad.position.y = 1;

        keypad.do_move(&Movement::Down);
        assert_eq!(keypad.position.y, 0);
        keypad.do_move(&Movement::Down);
        assert_eq!(keypad.position.y, -1);
        keypad.do_move(&Movement::Down);
        assert_eq!(keypad.position.y, -1);
    }

    #[test]
    fn test_move_left() {
        let mut keypad = Keypad::new();
        keypad.position.x = 1;

        keypad.do_move(&Movement::Left);
        assert_eq!(keypad.position.x, 0);
        keypad.do_move(&Movement::Left);
        assert_eq!(keypad.position.x, -1);
        keypad.do_move(&Movement::Left);
        assert_eq!(keypad.position.x, -1);
    }

    #[test]
    fn test_move_right() {
        let mut keypad = Keypad::new();
        keypad.position.x = -1;

        keypad.do_move(&Movement::Right);
        assert_eq!(keypad.position.x, 0);
        keypad.do_move(&Movement::Right);
        assert_eq!(keypad.position.x, 1);
        keypad.do_move(&Movement::Right);
        assert_eq!(keypad.position.x, 1);
    }

    #[test]
    fn test_press() {
        let mut keypad = Keypad::new();

        keypad.position = Position { x: -1, y: 1 };
        assert_eq!(1, keypad.press());

        keypad.position = Position { x: 0, y: 1 };
        assert_eq!(2, keypad.press());

        keypad.position = Position { x: 1, y: 1 };
        assert_eq!(3, keypad.press());

        keypad.position = Position { x: -1, y: 0 };
        assert_eq!(4, keypad.press());

        keypad.position = Position { x: 0, y: 0 };
        assert_eq!(5, keypad.press());

        keypad.position = Position { x: 1, y: 0 };
        assert_eq!(6, keypad.press());

        keypad.position = Position { x: -1, y: -1 };
        assert_eq!(7, keypad.press());

        keypad.position = Position { x: 0, y: -1 };
        assert_eq!(8, keypad.press());

        keypad.position = Position { x: 1, y: -1 };
        assert_eq!(9, keypad.press());
    }
}
