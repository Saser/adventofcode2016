extern crate base;
use base::Part;

extern crate day02;

fn get_answer(input: &str, part: Part) -> String {
    let solver = day02::get_solver();
    solver.solve(input, part).unwrap()
}

mod part1 {
    use super::*;

    fn get_answer_p1(input: &str) -> String {
        get_answer(input, Part::One)
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

mod part2 {
    use super::*;

    fn get_answer_p2(input: &str) -> String {
        get_answer(input, Part::Two)
    }

    fn assert_correct_answer(input: &str, expected: &str) {
        assert_eq!(&get_answer_p2(input), expected);
    }

    #[test]
    fn example1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_correct_answer(input, "5DB3");
    }
}
