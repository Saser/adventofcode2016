extern crate base;
use base::{Part, ProblemSolver};
use base::coord::{Direction, Position, Turn};

use std::collections::HashSet;
use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        let instructions = parse_input(input)?;
        match part {
            Part::One => solve_part_one(&instructions),
            Part::Two => solve_part_two(&instructions),
        }
    }
}

// Here starts the actual solution, lol

fn solve_part_one(instructions: &[Instruction]) -> Result<String, String> {
    let mut path = travel(instructions);
    match path.pop() {
        Some(position) => Ok(position.taxi_distance().to_string()),
        None => Err("something went wrong, the path was empty".to_owned()),
    }
}

fn solve_part_two(instructions: &[Instruction]) -> Result<String, String> {
    let path = travel(instructions);
    let mut visited = HashSet::new();
    for position in &path {
        if visited.contains(&position) {
            return Ok(position.taxi_distance().to_string());
        }

        visited.insert(position);
    }
    Err(format!("the path never crosses itself: {:?}", path))
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, String> {
    let instructions = input.trim().split(", ").map(Instruction::from_str);
    base::utils::any_err(instructions)
}

fn travel(instructions: &[Instruction]) -> Vec<Position> {
    let mut direction = Direction::Up;
    let mut position = Position::new();
    let mut path = vec![position];
    for instruction in instructions {
        direction.turn_mut(instruction.turn);
        for _ in 0..instruction.distance {
            position.walk_mut(direction);
            path.push(position);
        }
    }
    path
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
        let distance = u32::from_str(distance_str).map_err(|e| e.description().to_string())?;

        if distance == 0 {
            Err("distance must be positive".to_string())
        } else {
            Ok(Instruction {
                turn: turn,
                distance: distance,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
