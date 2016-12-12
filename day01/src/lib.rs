#![feature(inclusive_range_syntax)]

extern crate base;
use base::{Part, ProblemSolver};

use std::str::FromStr;
use std::ops::{Add, Mul};

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

fn solve_part_one(input: &str) -> Result<String, String> {
    let instructions = Instruction::parse_instructions(input)?;
    let traveler = Traveler::start_values();
    let final_traveler = traveler.apply_many_instructions(&instructions);
    Ok(final_traveler.position.distance().to_string())
}

pub struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: &base::Part) -> Result<String, String> {
        match *part {
            Part::One => solve_part_one(input),
            Part::Two => Err("not implemented yet".to_owned()),
        }
    }

    fn solve_file(&self, file_path: &str, part: &base::Part) -> Result<String, String> {
        let lines = base::utils::lines_from_file(file_path);
        match *part {
            Part::One => solve_part_one(&lines[0]),
            Part::Two => Err("not implemented yet".to_owned()),
        }
    }
}

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

    fn to_position_representation(&self) -> Position {
        match *self {
            Direction::North => Position { x: 0, y: 1 },
            Direction::East => Position { x: 1, y: 0 },
            Direction::South => Position { x: 0, y: -1 },
            Direction::West => Position { x: -1, y: 0 },
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

#[derive(Debug, Eq, PartialEq, Clone)]
struct Instruction {
    turn: Turn,
    distance: i32,
}

impl Instruction {
    fn parse_instructions(instructions_str: &str) -> Result<Vec<Instruction>, String> {
        instructions_str.split(", ")
            .map(Instruction::from_str)
            .filter(|x| x.is_ok())
            .collect()
    }

    fn normalize(&self) -> Instruction {
        Instruction { distance: 1, ..*self }
    }
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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
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

#[derive(Clone)]
struct Traveler {
    position: Position,
    direction: Direction,
}

impl Traveler {
    fn start_values() -> Traveler {
        Traveler {
            position: Position { x: 0, y: 0 },
            direction: Direction::North,
        }
    }

    fn apply_instruction(&self, instruction: &Instruction) -> Traveler {
        let new_dir = self.direction.turn(&instruction.turn);
        let travel_vector = new_dir.to_position_representation() * instruction.distance;
        Traveler {
            position: self.position.clone() + travel_vector,
            direction: new_dir,
        }
    }

    fn apply_many_instructions(&self, instructions: &[Instruction]) -> Traveler {
        instructions.iter()
            .fold(self.clone(),
                  |acc, instruction| acc.apply_instruction(instruction))
    }

    fn path_from_instruction(&self, instruction: &Instruction) -> Vec<Position> {
        let mut path = vec![];
        let normalized_vector = self.direction
            .turn(&instruction.turn)
            .to_position_representation();
        for d in 1...instruction.distance {
            let travel_vector = normalized_vector.clone() * d;
            let position = self.position.clone() + travel_vector;
            path.push(position);
        }
        path
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

    #[test]
    fn test_apply_instruction() {
        let traveler = Traveler::start_values();
        let i1 = Instruction::from_str("R2").unwrap();
        let i2 = Instruction::from_str("L3").unwrap();
        let new_traveler = traveler.apply_instruction(&i1).apply_instruction(&i2);

        assert_eq!(new_traveler.position, Position { x: 2, y: 3 });
        assert_eq!(new_traveler.direction, Direction::North);
    }

    #[test]
    fn test_apply_many_instructions() {
        let traveler = Traveler::start_values();
        let i1 = Instruction::from_str("R2").unwrap();
        let i2 = Instruction::from_str("L3").unwrap();
        let instructions = &[i1, i2];
        let new_traveler = traveler.apply_many_instructions(instructions);

        assert_eq!(new_traveler.position, Position { x: 2, y: 3 });
        assert_eq!(new_traveler.direction, Direction::North);
    }

    #[test]
    fn parse_instructions() {
        let instructions_str = "R2, L3";
        let instructions = Instruction::parse_instructions(instructions_str).unwrap();

        assert_eq!(instructions[0],
                   Instruction {
                       turn: Turn::Right,
                       distance: 2,
                   });
        assert_eq!(instructions[1],
                   Instruction {
                       turn: Turn::Left,
                       distance: 3,
                   });
    }

    #[test]
    fn path_from_instruction() {
        let traveler = Traveler::start_values();
        let instruction = Instruction::from_str("R5").unwrap();
        let path = traveler.path_from_instruction(&instruction);
        println!("{:?}", path);
        for x in 1..5 {
            assert!(path.contains(&Position { x: x, y: 0 }));
        }
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    fn get_answer(input: &str) -> u32 {
        let answer_str = solve_part_one(input).unwrap();
        u32::from_str(&answer_str).unwrap()
    }

    #[test]
    fn input_1() {
        let answer = get_answer("L2, R3");
        assert_eq!(answer, 5);
    }

    #[test]
    fn input_2() {
        let answer = get_answer("R2, R2, R2");
        assert_eq!(answer, 2);
    }

    #[test]
    fn input_3() {
        let answer = get_answer("R5, L5, R5, R3");
        assert_eq!(answer, 12);
    }
}
