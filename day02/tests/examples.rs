extern crate base;
use base::{Part, ProblemSolver};

extern crate day02;

use std::str::FromStr;

fn get_answer(input: &str, part: &Part) -> String {
    let solver = day02::get_solver();
    solver.solve(input, part).unwrap()
}

mod part1 {
    use super::*;

    fn get_answer_p1(input: &str) -> String {
        get_answer(input, &Part::One)
    }

    fn assert_correct_answer(input: &str, expected: &str) {
        assert_eq!(&get_answer_p1(input), expected);
    }

    #[test]
    fn example1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_correct_answer(input, "1985");
    }
}
