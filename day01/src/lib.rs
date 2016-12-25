extern crate base;
use base::{Part, ProblemSolver};
use base::coord::{Direction, Position, Turn};

use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        let input = input.trim();
        let instructions = parse_input(input);
        Err("herp derp".to_string())
    }
}

// Here starts the actual solution, lol

fn parse_input(input: &str) -> Result<Vec<Instruction>, String> {
    let instructions = input.split(", ").map(Instruction::from_str);
    base::utils::any_err(instructions)
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
