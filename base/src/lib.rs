pub enum Part {
    One,
    Two,
}

pub trait ProblemSolver {
    fn solve(input: &str) -> String;

    fn solve_file(file_path: &str) -> String;
}
