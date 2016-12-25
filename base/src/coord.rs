//! This module contains some useful functions, structs, and enum for working with discrete
//! coordinates in a plane.

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

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_parse_origin() {
        let origin_str = "(0, 0)";
        let origin = Position::from_str(origin_str).unwrap();
        assert_eq!(origin.0, 0);
        assert_eq!(origin.1, 0);
    }

    #[test]
    fn test_parse_origin_negative() {
        let origin_strs = ["(-0, 0)", "(0, -0)", "(-0, -0)"];
        for origin_str in &origin_strs {
            let origin = Position::from_str(origin_str).unwrap();
            assert_eq!(origin.0, 0);
            assert_eq!(origin.1, 0);
        }
    }

    #[test]
    fn test_parse_positve() {
        let pos_str = "(1, 10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(pos.0, 1);
        assert_eq!(pos.1, 10);
    }

    #[test]
    fn test_parse_one_negative() {
        let pos_str = "(-1, 10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(pos.0, -1);
        assert_eq!(pos.1, 10);

        let pos_str = "(1, -10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(pos.0, 1);
        assert_eq!(pos.1, -10);
    }

    #[test]
    fn test_parse_both_negative() {
        let pos_str = "(-1, -10)";
        let pos = Position::from_str(pos_str).unwrap();
        assert_eq!(pos.0, -1);
        assert_eq!(pos.1, -10);
    }

    #[test]
    fn test_different_spacing() {
        let pos_strs = ["(1,2)", "( 1, 2 )", "(      1, 2    )"];
        for pos_str in &pos_strs {
            let pos = Position::from_str(pos_str).unwrap();
            assert_eq!(pos.0, 1);
            assert_eq!(pos.1, 2);
        }
    }
}
