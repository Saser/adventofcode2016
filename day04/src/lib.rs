extern crate base;
use base::{Part, ProblemSolver};

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day04)
}

struct Day04;

impl ProblemSolver for Day04 {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        unimplemented!()
    }
}
