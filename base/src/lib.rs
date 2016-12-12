pub enum Part {
    One,
    Two,
}

pub trait ProblemSolver {
    fn new() -> Self;

    fn solve(input: &str, part: Part) -> String;

    fn solve_file(file_path: &str, part: Part) -> String;
}
