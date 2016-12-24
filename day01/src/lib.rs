extern crate base;
use base::{Part, ProblemSolver};

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: &Part) -> Result<String, String> {
        match *part {
            Part::One => solve_part_one(&input),
            Part::Two => solve_part_two(&input),
        }
    }

    fn solve_file(&self, file_path: &str, part: &Part) -> Result<String, String> {
        let lines = base::utils::lines_from_file(file_path);
        let input = &lines[0];
        match *part {
            Part::One => solve_part_one(input),
            Part::Two => solve_part_two(input),
        }
    }
}

fn solve_part_one(input: &str) -> Result<String, String> {
    Err("not implemented yet!".to_owned())
}

fn solve_part_two(input: &str) -> Result<String, String> {
    Err("not implemented yet!".to_owned())
}

// Here starts the actual solution, lol

use std::str::FromStr;
use std::ops::Add;

#[derive(Debug, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err(format!("not a valid turn: {}", s)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    turn: Turn,
    distance: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::error::Error;

        let (turn_str, distance_str) = s.split_at(1);
        let turn = Turn::from_str(turn_str)?;
        let distance = u32::from_str(distance_str).map_err(|e| e.description().to_owned())?;

        if distance == 0 {
            Err("distance must be positive".to_owned())
        } else {
            Ok(Instruction {
                turn: turn,
                distance: distance,
            })
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

    fn to_vector(&self) -> Position {
        match *self {
            Direction::North => Position { x: 0, y: 1 },
            Direction::East => Position { x: 1, y: 0 },
            Direction::South => Position { x: 0, y: -1 },
            Direction::West => Position { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Traveler {
    direction: Direction,
    position: Position,
}

impl Traveler {
    fn new() -> Traveler {
        Traveler {
            direction: Direction::North,
            position: Position { x: 0, y: 0 },
        }
    }

    fn turn(&mut self, turn: &Turn) {
        self.direction = self.direction.turn(turn);
    }

    fn walk(&mut self) {
        self.position = self.position.clone() + self.direction.to_vector();
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        self.turn(&instruction.turn);
        for _ in 0..instruction.distance {
            self.walk();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_from_str() {
        let right_str = "R";
        let turn = Turn::from_str(right_str).unwrap();
        assert_eq!(Turn::Right, turn);

        let left_str = "L";
        let turn = Turn::from_str(left_str).unwrap();
        assert_eq!(Turn::Left, turn);
    }

    #[test]
    fn turn_from_str_err() {
        let err_strs = ["r", "l", "right", "left"];
        for err_str in &err_strs {
            assert!(Turn::from_str(err_str).is_err());
        }
    }

    #[test]
    fn instruction_from_str() {
        let ins_str1 = "L2";
        let ins1 = Instruction::from_str(ins_str1).unwrap();
        assert_eq!(ins1,
                   Instruction {
                       turn: Turn::Left,
                       distance: 2,
                   });

        let ins_str2 = "R3";
        let ins2 = Instruction::from_str(ins_str2).unwrap();
        assert_eq!(ins2,
                   Instruction {
                       turn: Turn::Right,
                       distance: 3,
                   });
    }

    #[test]
    fn instruction_from_str_err() {
        let err_strs = ["L0", "R-1", "RL2"];
        for err_str in &err_strs {
            assert!(Instruction::from_str(err_str).is_err());
        }
    }
}
