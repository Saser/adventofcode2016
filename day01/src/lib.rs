extern crate base;
use base::{Part, ProblemSolver};

struct Solver;

impl ProblemSolver for Solver {
    fn solve(&self, input: &str, part: &Part) -> Result<String, String> {
        Err("not implemented yet!".to_owned())
    }

    fn solve_file(&self, file_path: &str, part: &Part) -> Result<String, String> {
        Err("not implemented yet!".to_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
