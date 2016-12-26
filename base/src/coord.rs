//! This module contains some useful functions, structs, and enum for working with discrete
//! coordinates in a plane.

use regex::Regex;

use std::ops::Add;
use std::str::FromStr;

use ::FromChar;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Turn {
    Left,
    Right,
}

impl FromChar for Turn {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            'L' | 'l' => Ok(Turn::Left),
            'R' | 'r' => Ok(Turn::Right),
            _ => Err(format!("not a valid turn: {}", c)),
        }
    }
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" | "l" => Ok(Turn::Left),
            "R" | "r" => Ok(Turn::Right),
            _ => Err(format!("not a valid turn: {}", s)),
        }
    }
}

#[cfg(test)]
mod turn_tests {
    use super::*;

    #[test]
    fn test_parse_char_left() {
        let left_chars = ['L', 'l'];
        for left_char in &left_chars {
            assert_eq!(Turn::Left, Turn::from_char(*left_char).unwrap());
        }
    }

    #[test]
    fn test_parse_str_left() {
        let left_strs = ["L", "l"];
        for left_str in &left_strs {
            assert_eq!(Turn::Left, Turn::from_str(left_str).unwrap());
        }
    }

    #[test]
    fn test_parse_char_right() {
        let right_chars = ['R', 'r'];
        for right_char in &right_chars {
            assert_eq!(Turn::Right, Turn::from_char(*right_char).unwrap());
        }
    }

    #[test]
    fn test_parse_str_right() {
        let right_strs = ["R", "r"];
        for right_str in &right_strs {
            assert_eq!(Turn::Right, Turn::from_str(right_str).unwrap());
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromChar for Direction {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            'U' | 'u' => Ok(Direction::Up),
            'R' | 'r' => Ok(Direction::Right),
            'D' | 'd' => Ok(Direction::Down),
            'L' | 'l' => Ok(Direction::Left),
            _ => Err(format!("not a valid direction: {}", c)),
        }
    }
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

impl Direction {
    pub fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Right => self.turn_right(),
            Turn::Left => self.turn_left(),
        }
    }

    pub fn turn_mut(&mut self, turn: Turn) {
        *self = self.turn(turn);
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn test_parse_char_up() {
        let up_chars = ['U', 'u'];
        for up_char in &up_chars {
            assert_eq!(Direction::Up, Direction::from_char(*up_char).unwrap());
        }
    }

    #[test]
    fn test_parse_str_up() {
        let up_strs = ["U", "u"];
        for up_str in &up_strs {
            assert_eq!(Direction::Up, Direction::from_str(up_str).unwrap());
        }
    }

    #[test]
    fn test_parse_char_right() {
        let right_chars = ['R', 'r'];
        for right_char in &right_chars {
            assert_eq!(Direction::Right, Direction::from_char(*right_char).unwrap());
        }
    }

    #[test]
    fn test_parse_str_right() {
        let right_strs = ["R", "r"];
        for right_str in &right_strs {
            assert_eq!(Direction::Right, Direction::from_str(right_str).unwrap());
        }
    }

    #[test]
    fn test_parse_char_down() {
        let down_chars = ['D', 'd'];
        for down_char in &down_chars {
            assert_eq!(Direction::Down, Direction::from_char(*down_char).unwrap());
        }
    }

    #[test]
    fn test_parse_str_down() {
        let down_strs = ["D", "d"];
        for down_str in &down_strs {
            assert_eq!(Direction::Down, Direction::from_str(down_str).unwrap());
        }
    }

    #[test]
    fn test_parse_char_left() {
        let left_chars = ['L', 'l'];
        for left_char in &left_chars {
            assert_eq!(Direction::Left, Direction::from_char(*left_char).unwrap());
        }
    }

    #[test]
    fn test_parse_str_left() {
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

    #[test]
    fn test_turn_right() {
        let direction = Direction::Up;
        let new_direction = direction.turn(Turn::Right);
        assert_eq!(new_direction, Direction::Right);
        let new_direction = new_direction.turn(Turn::Right);
        assert_eq!(new_direction, Direction::Down);
        let new_direction = new_direction.turn(Turn::Right);
        assert_eq!(new_direction, Direction::Left);
        let new_direction = new_direction.turn(Turn::Right);
        assert_eq!(new_direction, Direction::Up);
    }

    #[test]
    fn test_turn_mut_right() {
        let mut direction = Direction::Up;
        direction.turn_mut(Turn::Right);
        assert_eq!(direction, Direction::Right);
        direction.turn_mut(Turn::Right);
        assert_eq!(direction, Direction::Down);
        direction.turn_mut(Turn::Right);
        assert_eq!(direction, Direction::Left);
        direction.turn_mut(Turn::Right);
        assert_eq!(direction, Direction::Up);
    }

    #[test]
    fn test_turn_left() {
        let direction = Direction::Up;
        let new_direction = direction.turn(Turn::Left);
        assert_eq!(new_direction, Direction::Left);
        let new_direction = new_direction.turn(Turn::Left);
        assert_eq!(new_direction, Direction::Down);
        let new_direction = new_direction.turn(Turn::Left);
        assert_eq!(new_direction, Direction::Right);
        let new_direction = new_direction.turn(Turn::Left);
        assert_eq!(new_direction, Direction::Up);
    }

    #[test]
    fn test_turn_mut_left() {
        let mut direction = Direction::Up;
        direction.turn_mut(Turn::Left);
        assert_eq!(direction, Direction::Left);
        direction.turn_mut(Turn::Left);
        assert_eq!(direction, Direction::Down);
        direction.turn_mut(Turn::Left);
        assert_eq!(direction, Direction::Right);
        direction.turn_mut(Turn::Left);
        assert_eq!(direction, Direction::Up);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Position(i32, i32);

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
    pub fn new() -> Position {
        Position(0, 0)
    }

    pub fn walk_n(&self, direction: Direction, n: u32) -> Position {
        let dx = match direction {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        } * n as i32;

        let dy = match direction {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        } * n as i32;

        let vector = Position(dx, dy);

        self.clone() + vector
    }

    pub fn walk(&self, direction: Direction) -> Position {
        self.walk_n(direction, 1)
    }

    pub fn walk_n_mut(&mut self, direction: Direction, n: u32) {
        *self = self.walk_n(direction, n);
    }

    pub fn walk_mut(&mut self, direction: Direction) {
        self.walk_n_mut(direction, 1);
    }

    pub fn taxi_distance(&self) -> u32 {
        (self.0.abs() + self.1.abs()) as u32
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
        let new_pos = pos.walk_n(Direction::Up, 3);
        assert_eq!(Position(0, 3), new_pos);
        let new_pos = new_pos.walk_n(Direction::Right, 4);
        assert_eq!(Position(4, 3), new_pos);
        let new_pos = new_pos.walk_n(Direction::Left, 8);
        assert_eq!(Position(-4, 3), new_pos);
        let new_pos = new_pos.walk_n(Direction::Down, 4);
        assert_eq!(Position(-4, -1), new_pos);
    }

    #[test]
    fn test_walk_mut_n() {
        let mut pos = Position::new();
        pos.walk_n_mut(Direction::Up, 3);
        assert_eq!(Position(0, 3), pos);
        pos.walk_n_mut(Direction::Right, 4);
        assert_eq!(Position(4, 3), pos);
        pos.walk_n_mut(Direction::Left, 8);
        assert_eq!(Position(-4, 3), pos);
        pos.walk_n_mut(Direction::Down, 4);
        assert_eq!(Position(-4, -1), pos);
    }

    #[test]
    fn test_walk() {
        let pos = Position::new();
        let new_pos = pos.walk(Direction::Up);
        assert_eq!(Position(0, 1), new_pos);
        let new_pos = new_pos.walk(Direction::Right);
        assert_eq!(Position(1, 1), new_pos);
        let new_pos = new_pos.walk(Direction::Left);
        assert_eq!(Position(0, 1), new_pos);
        let new_pos = new_pos.walk(Direction::Down);
        assert_eq!(Position(0, 0), new_pos);
    }

    #[test]
    fn test_walk_mut() {
        let mut pos = Position::new();
        pos.walk_mut(Direction::Up);
        assert_eq!(Position(0, 1), pos);
        pos.walk_mut(Direction::Right);
        assert_eq!(Position(1, 1), pos);
        pos.walk_mut(Direction::Left);
        assert_eq!(Position(0, 1), pos);
        pos.walk_mut(Direction::Down);
        assert_eq!(Position(0, 0), pos);
    }

    #[test]
    fn test_taxi_distance_positive() {
        let pos = Position(2, 3);
        assert_eq!(pos.taxi_distance(), 5);
    }

    #[test]
    fn test_taxi_distance_one_negative() {
        let pos = Position(-2, 3);
        assert_eq!(pos.taxi_distance(), 5);

        let pos = Position(2, -3);
        assert_eq!(pos.taxi_distance(), 5);
    }

    #[test]
    fn test_taxi_distance_negative() {
        let pos = Position(-2, -3);
        assert_eq!(pos.taxi_distance(), 5);
    }
}
