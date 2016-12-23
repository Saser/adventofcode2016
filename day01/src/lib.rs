extern crate base;
use base::{Part, ProblemSolver};

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
