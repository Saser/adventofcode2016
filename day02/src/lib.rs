extern crate base;
use base::{Part, ProblemSolver};

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: &Part) -> Result<String, String> {
        let lines = input.split('\n')
            .map(str::to_string)
            .collect::<Vec<String>>();
        match *part {
            Part::One => solve_part_one(&lines),
            Part::Two => solve_part_two(&lines),
        }
    }

    fn solve_file(&self, file_path: &str, part: &Part) -> Result<String, String> {
        let lines = base::utils::lines_from_file(file_path);
        match *part {
            Part::One => solve_part_one(&lines),
            Part::Two => solve_part_two(&lines),
        }
    }
}

// Here starts the actual solution

use std::str::FromStr;

fn solve_part_one(input_lines: &[String]) -> Result<String, String> {
    unimplemented!()
}

fn solve_part_two(input_lines: &[String]) -> Result<String, String> {
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
