extern crate base;
use base::{Part, ProblemSolver};

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day03)
}

struct Day03;

impl ProblemSolver for Day03 {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        unimplemented!()
    }
}
