extern crate base;
use base::{Part, ProblemSolver};

extern crate day01;

use std::str::FromStr;

fn get_answer(input: &str, part: &Part) -> u32 {
    let solver = day01::get_solver();
    let answer_str = &solver.solve(input, part).unwrap();
    u32::from_str(answer_str).unwrap()
}

mod part1 {
    use super::*;

    fn get_answer_p1(input: &str) -> u32 {
        get_answer(input, &Part::One)
    }

    fn correct_answer(input: &str, expected: u32) -> bool {
        get_answer_p1(input) == expected
    }

    #[test]
    fn example1() {
        assert!(correct_answer("R2, L3", 5));
    }

    #[test]
    fn example2() {
        assert!(correct_answer("R2, R2, R2", 2));
    }

    #[test]
    fn example3() {
        assert!(correct_answer("R5, L5, R5, R3", 12));
    }
}
