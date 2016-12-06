use std::str::FromStr;
use std::ops::{Add, Mul};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Direction {
        match *turn {
            Turn::Right => self.turn_right(),
            Turn::Left => self.turn_left(),
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Turn {
    Right,
    Left,
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Turn::Right),
            "L" => Ok(Turn::Left),
            _ => Err(format!("invalid turn: {}", s)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    turn: Turn,
    distance: i32,
}


impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let turn_char = chars.next().ok_or("invalid direction".to_owned())?;
        let turn = Turn::from_str(&turn_char.to_string())?;
        let distance_str = chars.as_str();
        let distance = i32::from_str(distance_str).map_err(|_| "invalid distance")?;
        Ok(Instruction {
            turn: turn,
            distance: distance,
        })
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Position;

    fn mul(self, rhs: i32) -> Position {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn many_turns(init_dir: &Direction, turns: &[Turn]) -> Direction {
        turns.iter()
            .fold(init_dir.clone(), |acc, turn| acc.turn(turn))
    }

    #[test]
    fn turn_right_then_left() {
        let dir = Direction::North;
        let newdir = many_turns(&dir, &[Turn::Right, Turn::Left]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_left_then_right() {
        let dir = Direction::North;
        let newdir = many_turns(&dir, &[Turn::Left, Turn::Right]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_right_four_times() {
        let dir = Direction::East;
        let newdir = many_turns(&dir, &[Turn::Right; 4]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_left_four_times() {
        let dir = Direction::West;
        let newdir = many_turns(&dir, &[Turn::Left; 4]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_fromstr() {
        let right = Turn::from_str("R").unwrap();
        let left = Turn::from_str("L").unwrap();
        let invalid = Turn::from_str("invalid");

        assert_eq!(right, Turn::Right);
        assert_eq!(left, Turn::Left);
        assert!(invalid.is_err());
    }

    #[test]
    fn instruction_fromstr() {
        let i1 = Instruction::from_str("L14").unwrap();
        assert_eq!(i1,
                   Instruction {
                       turn: Turn::Left,
                       distance: 14,
                   });

        let i2 = Instruction::from_str("R14").unwrap();
        assert_eq!(i2,
                   Instruction {
                       turn: Turn::Right,
                       distance: 14,
                   });
    }
}
