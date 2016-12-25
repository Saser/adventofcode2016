//! This module contains some useful functions, structs, and enum for working with discrete
//! coordinates in a plane.

use std::ops::Add;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "u" => Ok(Direction::Up),
            "R" | "r" => Ok(Direction::Right),
            "D" | "d" => Ok(Direction::Down),
            "L" | "l" => Ok(Direction::Left),
            _ => Err(format!("not a valid direction: {}", s)),
        }
    }
}

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn test_parse_up() {
        let up_strs = ["U", "u"];
        for up_str in &up_strs {
            assert_eq!(Direction::Up, Direction::from_str(up_str).unwrap());
        }
    }

    #[test]
    fn test_parse_right() {
        let right_strs = ["R", "r"];
        for right_str in &right_strs {
            assert_eq!(Direction::Right, Direction::from_str(right_str).unwrap());
        }
    }

    #[test]
    fn test_parse_down() {
        let down_strs = ["D", "d"];
        for down_str in &down_strs {
            assert_eq!(Direction::Down, Direction::from_str(down_str).unwrap());
        }
    }

    #[test]
    fn test_parse_left() {
        let left_strs = ["L", "l"];
        for left_str in &left_strs {
            assert_eq!(Direction::Left, Direction::from_str(left_str).unwrap());
        }
    }

    #[test]
    fn test_parse_err() {
        let err_strs = ["Left", "Right", " Up", "Down "];
        for err_str in &err_strs {
            assert!(Direction::from_str(err_str).is_err());
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Position(i32, i32);

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::error::Error;

        let re = Regex::new(r"^\( *(?P<x>-?\d+), *(?P<y>-?\d+) *\)$").unwrap();
        let captures = re.captures(s).ok_or(format!("invalid position string: {}", s))?;

        let x_str = captures.name("x").unwrap();
        let x = i32::from_str(x_str).unwrap();

        let y_str = captures.name("y").unwrap();
        let y = i32::from_str(y_str).unwrap();

        Ok(Position(x, y))
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Position {
    fn new() -> Position {
        Position(0, 0)
    }

    fn walk_n(&self, direction: &Direction, n: u32) -> Position {
        let dx = match *direction {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        } * n as i32;

        let dy = match *direction {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        } * n as i32;

        let vector = Position(dx, dy);

        self.clone() + vector
    }

    fn walk(&self, direction: &Direction) -> Position {
        self.walk_n(direction, 1)
    }

    fn walk_n_mut(&mut self, direction: &Direction, n: u32) {
        *self = self.clone().walk_n(direction, n);
    }

    fn walk_mut(&mut self, direction: &Direction) {
        self.walk_n_mut(direction, 1);
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_parse_origin() {
        let origin_str = "(0, 0)";
        let origin = Position::from_str(origin_str).unwrap();
        assert_eq!(Position(0, 0), origin);
    }

    #[test]
    fn test_parse_origin_negative() {
        let origin_strs = ["(-0, 0)", "(0, -0)", "(-0, -0)"];
        for origin_str in &origin_strs {
            let origin = Position::from_str(origin_str).unwrap();
            assert_eq!(Position(0, 0), origin);
        }
    }

    #[test]
    fn test_parse_positve() {
        let pos_str = "(1, 10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(Position(1, 10), pos);
    }

    #[test]
    fn test_parse_one_negative() {
        let pos_str = "(-1, 10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(Position(-1, 10), pos);

        let pos_str = "(1, -10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(Position(1, -10), pos);
    }

    #[test]
    fn test_parse_both_negative() {
        let pos_str = "(-1, -10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(Position(-1, -10), pos);
    }

    #[test]
    fn test_different_spacing() {
        let pos_strs = ["(1,2)", "( 1, 2 )", "(      1, 2    )"];
        for pos_str in &pos_strs {
            let pos = Position::from_str(pos_str).unwrap();
            assert_eq!(Position(1, 2), pos);
        }
    }

    #[test]
    fn test_add_positive() {
        let pos1 = Position(1, 2);
        let pos2 = Position(2, 3);
        let new_pos = pos1 + pos2;
        assert_eq!(Position(3, 5), new_pos);
    }

    #[test]
    fn test_add_negative() {
        let pos1 = Position(1, 2);
        let pos2 = Position(-2, -3);
        let new_pos = pos1 + pos2;
        assert_eq!(Position(-1, -1), new_pos);
    }

    #[test]
    fn test_walk_n() {
        let pos = Position::new();
        let new_pos = pos.walk_n(&Direction::Up, 3);
        assert_eq!(Position(0, 3), new_pos);
        let new_pos = new_pos.walk_n(&Direction::Right, 4);
        assert_eq!(Position(4, 3), new_pos);
        let new_pos = new_pos.walk_n(&Direction::Left, 8);
        assert_eq!(Position(-4, 3), new_pos);
        let new_pos = new_pos.walk_n(&Direction::Down, 4);
        assert_eq!(Position(-4, -1), new_pos);
    }

    #[test]
    fn test_walk_mut_n() {
        let mut pos = Position::new();
        pos.walk_n_mut(&Direction::Up, 3);
        assert_eq!(Position(0, 3), pos);
        pos.walk_n_mut(&Direction::Right, 4);
        assert_eq!(Position(4, 3), pos);
        pos.walk_n_mut(&Direction::Left, 8);
        assert_eq!(Position(-4, 3), pos);
        pos.walk_n_mut(&Direction::Down, 4);
        assert_eq!(Position(-4, -1), pos);
    }

    #[test]
    fn test_walk() {
        let pos = Position::new();
        let new_pos = pos.walk(&Direction::Up);
        assert_eq!(Position(0, 1), new_pos);
        let new_pos = new_pos.walk(&Direction::Right);
        assert_eq!(Position(1, 1), new_pos);
        let new_pos = new_pos.walk(&Direction::Left);
        assert_eq!(Position(0, 1), new_pos);
        let new_pos = new_pos.walk(&Direction::Down);
        assert_eq!(Position(0, 0), new_pos);
    }

    #[test]
    fn test_walk_mut() {
        let mut pos = Position::new();
        pos.walk_mut(&Direction::Up);
        assert_eq!(Position(0, 1), pos);
        pos.walk_mut(&Direction::Right);
        assert_eq!(Position(1, 1), pos);
        pos.walk_mut(&Direction::Left);
        assert_eq!(Position(0, 1), pos);
        pos.walk_mut(&Direction::Down);
        assert_eq!(Position(0, 0), pos);
    }
}
