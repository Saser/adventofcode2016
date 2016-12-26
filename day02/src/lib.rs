extern crate base;
use base::{Part, ProblemSolver};
use base::coord::Position;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Solver)
}

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        unimplemented!()
    }
}

trait Keypad {
    fn press(position: Position) -> Option<String>;
}

struct StandardKeypad;

impl Keypad for StandardKeypad {
    fn press(position: Position) -> Option<String> {
        match position {
            Position(-1, 1) => Some(1.to_string()),
            Position(0, 1) => Some(2.to_string()),
            Position(1, 1) => Some(3.to_string()),
            Position(-1, 0) => Some(4.to_string()),
            Position(0, 0) => Some(5.to_string()),
            Position(1, 0) => Some(6.to_string()),
            Position(-1, -1) => Some(7.to_string()),
            Position(0, -1) => Some(8.to_string()),
            Position(1, -1) => Some(9.to_string()),
            _ => None,
        }
    }
}
